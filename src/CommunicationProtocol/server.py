import time
import zmq
import joblib
import samples_pb2 as samples_pb2
from pathlib import Path

context = zmq.Context()
socket = context.socket(zmq.REP)
socket.bind("tcp://*:5556")
cf_model = joblib.load('../models/front_facing_with.pkl')
nocf_model = joblib.load('../models/front_facing_without.pkl')

sample = samples_pb2.Sample()
classification = samples_pb2.Classification()

while True:
    #  Wait for next request from client
    message = socket.recv()
    result = sample.FromString(message)
    print("Received request: %s" % result)
    # print("Received request: %s" % str(message))
    results = []
    for i, value in result.ListFields():
        results.append(value)
        
    # test = pd.DataFrame([results], columns=["cfDNA_ng_mL_plasma",
    #             "Albumin (g/L)",
    #             "LDH (ULN)",
    #             "ALP (ULN)",
    #             "PSA (ng/ml)",
    #             "Liver mets",
    #             "Lung mets",
    #             "ECOG PS",])
    
    print(results)
    print(cf_model.predict(results))
    # classification.label = model.predict(test)[0]
    
    
    #  Do some 'work'
    time.sleep(1)

    #  Send reply back to client
    socket.send(classification.SerializeToString())
    
    # socket.send(b'244')
    
    
            #  "cfDNA_ng_mL_plasma",
            #     "Albumin (g/L)",
            #     "LDH (ULN)",
            #     "ALP (ULN)",
            #     "PSA (ng/ml)",
            #     "Liver mets",
            #     "Lung mets",
            #     "ECOG PS",
    
    