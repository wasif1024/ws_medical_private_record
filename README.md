# Medical Private Record - Confidential Computing on Solana with Arcium

![Private Data Confidential Computing](assets/private_data.png)

A Solana-based application leveraging **Arcium's confidential computing network** to enable secure, privacy-preserving storage and sharing of sensitive medical records. This project demonstrates how blockchain technology, combined with multi-party computation (MPC), can facilitate secure data processing while maintaining strict privacy guarantees.

## Overview

This project implements an encrypted medical record system on the Solana blockchain using Arcium's confidential computing infrastructure. Medical data is stored in an encrypted format, and computations on sensitive patient information are performed off-chain using Arcium's secure co-processor network, ensuring that data remains private throughout the entire lifecycle.

## What is Arcium?

**Arcium** is a parallelized confidential computing network built for Solana that enables secure computation on encrypted data without ever exposing the underlying information. By leveraging advanced encryption techniques and multi-party computation (MPC), Arcium acts as a secure co-processor that allows blockchain applications to perform complex operations on sensitive data while maintaining complete privacy.

### Key Benefits of Arcium

1. **Enhanced Data Privacy**
   - Data remains encrypted during processing, never exposed in plaintext
   - Mitigates risks associated with data breaches and unauthorized access
   - Enables privacy-preserving computations on sensitive information

2. **Regulatory Compliance**
   - Assists organizations in adhering to stringent data protection regulations (GDPR, HIPAA, etc.)
   - Facilitates lawful data processing practices while maintaining confidentiality
   - Provides audit trails without compromising privacy

3. **Secure Multi-Party Collaboration**
   - Enables multiple parties to collaboratively process data without sharing raw datasets
   - Fosters partnerships while preserving individual data privacy
   - Allows for secure data aggregation and analysis across organizations

4. **Scalable Confidential Computing**
   - Parallelized MPC network architecture for high performance
   - Accommodates large-scale applications without compromising security
   - Cost-effective solution for confidential computing workloads

5. **Trustless Privacy**
   - No need to trust a single central authority
   - Cryptographic guarantees ensure data privacy
   - Transparent and verifiable computation results

## Use Cases

### 1. Private Health Data Management

This project specifically demonstrates how Arcium can be used to securely manage private health records:

- **Secure Storage**: Patient medical data (age, gender, blood type, weight, height, allergies) is stored in encrypted format on-chain
- **Privacy-Preserving Lookups**: Authorized parties can query patient records without exposing sensitive information
- **Compliance**: Enables healthcare providers to share patient data for research and treatment purposes while maintaining HIPAA compliance
- **Interoperability**: Facilitates secure data sharing between hospitals, clinics, and research institutions

**Benefits for Healthcare:**
- Patients maintain control over their medical data
- Healthcare providers can access necessary information securely
- Researchers can perform analytics on encrypted datasets
- Reduces the risk of data breaches and unauthorized access

### 2. Secure Banking Data Sharing

Arcium's confidential computing capabilities can revolutionize financial data sharing:

- **Fraud Detection**: Banks can collaboratively analyze transaction patterns across institutions without sharing raw customer data
- **Risk Assessment**: Financial institutions can pool data for credit scoring and risk modeling while preserving individual customer privacy
- **Regulatory Reporting**: Enable compliance with financial regulations while maintaining customer data confidentiality
- **Anti-Money Laundering (AML)**: Cross-institutional collaboration for AML detection without exposing sensitive transaction details

**Benefits for Banking:**
- Enhanced fraud detection through collaborative analysis
- Improved risk models with access to larger datasets
- Regulatory compliance without compromising customer privacy
- Reduced operational costs through secure data sharing

### 3. Privacy-Preserving AI Model Training

Arcium enables training of AI models on encrypted datasets without exposing raw data:

- **Federated Learning**: Train models across multiple data sources without centralizing sensitive information
- **Private Model Training**: Develop robust AI models on encrypted medical, financial, or personal data
- **Collaborative ML**: Multiple organizations can contribute to model training without revealing their datasets
- **Secure Inference**: Perform model predictions on encrypted input data

**Benefits for AI/ML:**
- Access to larger, more diverse training datasets
- Maintain data privacy throughout the ML lifecycle
- Enable collaboration between organizations with sensitive data
- Comply with data protection regulations in AI development
- Protect proprietary datasets while benefiting from collective intelligence

## Project Structure

This project follows a structure similar to a standard Solana Anchor project, with an additional component for confidential computations:

```
.
├── programs/                          # Standard Solana Anchor program
│   └── ws_medical_private_record/
│       └── src/
│           └── lib.rs                 # Main program logic
├── encrypted-ixs/                     # Confidential computing instructions
│   └── src/
│       └── lib.rs                     # Encrypted computation definitions
├── tests/                             # Test suite
└── Anchor.toml                        # Anchor configuration
```

### Key Components

1. **Programs Directory** (`programs/`)
   - Contains the main Solana program logic
   - Handles on-chain state management
   - Manages initialization and callback functions for confidential computations

2. **Encrypted Instructions Directory** (`encrypted-ixs/`)
   - Defines confidential computing operations using Arcis (Arcium's Rust framework)
   - Specifies how encrypted data should be processed
   - Contains the actual computation logic that runs in the secure enclave

### How It Works

1. **Data Storage**: Patient data is stored in encrypted format using the `store_patient_data` instruction
2. **Confidential Computation**: The `private_record_lookup` instruction initiates an off-chain computation
3. **Secure Processing**: Arcium's network processes the encrypted data in a secure enclave
4. **Result Delivery**: The computation results are returned via a callback function, maintaining encryption throughout

## Features

- 🔐 **End-to-End Encryption**: Data remains encrypted from storage through computation
- 🏥 **Medical Record Management**: Store and retrieve patient medical information securely
- 🔍 **Privacy-Preserving Queries**: Look up records without exposing sensitive data
- 🔗 **Blockchain Transparency**: All transactions are verifiable on-chain while data remains private
- ⚡ **Scalable**: Leverages Arcium's parallelized network for high performance

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Solana CLI (latest version)
- Anchor Framework (latest version)
- Arcium CLI tools

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd ws_medical_private_record
```

2. Install dependencies:
```bash
anchor build
```

3. Configure Arcium:
```toml
# Arcium.toml configuration
[localnet]
nodes = 2
localnet_timeout_secs = 60
backends = ["Cerberus"]
```

4. Run tests:
```bash
anchor test
```

## Program Instructions

### `store_patient_data`

Stores encrypted patient medical information on-chain.

**Parameters:**
- `patient_id`: Encrypted unique patient identifier
- `age`: Encrypted patient age
- `gender`: Encrypted gender information
- `blood_type`: Encrypted blood type
- `weight`: Encrypted weight measurement
- `height`: Encrypted height measurement
- `allergies`: Array of encrypted allergy information (up to 5)

### `private_record_lookup`

Initiates a confidential computation to look up and share patient records securely.

**Parameters:**
- `computation_offset`: Unique identifier for the computation
- `receiver`: X25519 public key of the receiving party
- `receiver_nonce`: Nonce for the receiver
- `sender_pub_key`: X25519 public key of the sender
- `nonce`: Nonce for the computation

### `init_private_record_lookup_comp_def`

Initializes the computation definition for private record lookups.

## Technical Details

- **Blockchain**: Solana
- **Framework**: Anchor
- **Confidential Computing**: Arcium Network
- **MPC Backend**: Cerberus
- **Encryption**: X25519 key exchange with Rescue cipher

## Security Considerations

- All sensitive data is encrypted before being stored on-chain
- Computations are performed in secure enclaves off-chain
- Data is never exposed in plaintext during processing
- Cryptographic proofs ensure computation integrity
- Access control is enforced through cryptographic keys

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Specify your license here]

## Resources

- [Arcium Documentation](https://docs.arcium.com)
- [Solana Documentation](https://docs.solana.com)
- [Anchor Framework](https://www.anchor-lang.com)

## Contact

[Your contact information]
