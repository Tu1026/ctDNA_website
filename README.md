# ctDNA_website
This website is built using the Rocket framework with Rust interacting with ML model trained using Ray and Xgboost. The cross-language communication is acheived using protobuf and zeromq.

My implementation of a website that uses Rust and simple javascript to host a machine learning model that is trained on 700+ metastatic prostate cancer patients

# ctdna_predict

Source code for the ctdna.org website

# Installation for python requirments

... activate environment ...
pip install -r requirements.txt

# Starting the server

... activate environment ...
python src/CommunicationProtocol/server.py
cargo run
