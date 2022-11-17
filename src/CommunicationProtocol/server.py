import time
import zmq
import joblib
import samples_pb2 as samples_pb2
import numpy as np
from pathlib import Path


def initial_zmp(port=5556):
    context = zmq.Context()
    socket = context.socket(zmq.REP)
    socket.bind(f"tcp://*:{port}")
    return socket

def load_models():
    cf_model = joblib.load('../models/front_facing_with.pkl')
    nocf_model = joblib.load('../models/front_facing_without.pkl')
    return cf_model, nocf_model

def load_protobuf():
    sample = samples_pb2.Sample()
    classification = samples_pb2.Classification()
    return sample, classification

"""
#! Quite dangeous usage of protobuf here as it right now I am initailizing
#! null values to -1 because protobuf cannot serialize null values for clarity
#! of the program. In the future if negative values are allowed for any reason
#! this will have to be revised (possibly serialize null to empty string \xa0)
"""
def listen_for_message(socket, sample, classification, cf_model, nocf_model):
    while True:
        #  Wait for next request from client
        message = socket.recv()
        result = sample.FromString(message)
        print("Received request: %s" % result)
        # print("Received request: %s" % str(message))
        

        results = [result.cfdna_yield, result.psa, result.ldh, result.alp, result.albumin, 
                 result.ecog, result.liver_mets, result.lung_mets,]
        
        print(f"cfDNA:{result.cfdna_yield}")
        
        if result.cfdna_yield != -1 :
            model = cf_model
            results = [result.cfdna_yield, result.psa, result.ldh, result.alp, result.albumin, 
                result.ecog, result.liver_mets, result.lung_mets,]
        
        
        else:
            model = nocf_model
            results = [result.psa, result.ldh, result.alp, result.albumin, 
                result.ecog, result.liver_mets, result.lung_mets,]
        
        
        results = [np.nan if x == -1 else x for x in results]
         
        print(results)
        print(model.predict_proba([results]))
        
        classification.label = model.predict([results])[0]
        classification.positive_proba = model.predict_proba([results])[0][1]
        classification.negative_proba = model.predict_proba([results])[0][0]
        
        print(classification)
            #  Send reply back to client
        socket.send(classification.SerializeToString())
        
        

def main():
    socket = initial_zmp()
    cf_model, nocf_model = load_models()
    sample, classification = load_protobuf()
    listen_for_message(socket, sample, classification, cf_model, nocf_model)
    
if __name__ == "__main__":
    main()
    