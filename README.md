<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/lorenzoronzani/dao-forge">
    <img src="docs/images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">dao-forge</h3>

  <p align="center">
    A decentralized platform for creating and managing DAOs (Decentralized Autonomous Organizations) in a legal and compliant way, built entirely on the Internet Computer Protocol (ICP).
    <br />
    <a href="https://github.com/lorenzoronzani/dao-forge/docs"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://yqq6b-aqaaa-aaaaj-qnr2q-cai.icp0.io/">View Demo</a>
    ·
    <a href="https://github.com/lorenzoronzani/dao-forge/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    ·
    <a href="https://github.com/lorenzoronzani/dao-forge/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->
## About The Project

DAO Forge represents a new paradigm in decentralized governance, bridging the gap between traditional organizational structures and blockchain-based autonomous systems.

DAO Forge enables governments and organizations to create legally compliant DAOs with an innovative voting process. The basic idea is that you are able to verify and track what happens from the creation of the poll, to the voting section, the automatic execution of a custom action based on the voting results and the tracking if the action has already been executed or not. The platform is fully decentralized, hosted on ICP, and designed to eliminate single points of failure while providing robust governance mechanisms.

Key features:
- Transparency: Full-stack and decentralized deployment ensuring complete visibility
- Interoperability: Seamless integration between off-chain and on-chain operations
- Flexibility: Extensible architecture for adding new functionalities and governance models
- Legal compliance: Built-in compliance with Swiss laws and regulatory frameworks
- Automation & verifiability: Automated execution of actions with full audit trails and verification capabilities

This comprehensive approach ensures that organizations can leverage the benefits of decentralized governance while maintaining the legal certainty and operational efficiency required for real-world applications.

### Built with

The tech stack of this project includes:

[![ICP][ICP]][ICP-url] 
<br>
[![Rust][Rust]][Rust-url]
<br>
[![React][React]][React-url]
<br>
[![Shadcn][Shadcn]][Shadcn-url]
<br>
[![Typescript][Typescript]][Typescript-url]
<br>

### ICP architecture

The DAO Forge platform is built on a modular, decentralized architecture composed of specialized canisters deployed across the ICP. This design ensures high availability, verifiability, and legal compliance while maintaining the flexibility to support different jurisdictions and DAO structures.

<div align="center">
  <a href="https://github.com/IsinBlockchainTeam/flutter_isin_blockchain_wallet_manager">
    <img src="docs/images/architecture.png" alt="Architecture">
  </a>
</div>

The architecture is composed of the following canisters:
- dao_agency: Orchestrates DAO creation and deployment of legal-form-specific canisters
- dao_association: Company in the schema, represents individual DAO instances with all operations and data for specific legal forms (currently Swiss associations)
- voting: Central governance engine handling the complete voting lifecycle, from creation to automated execution
- dao_discovery: Cross-platform registry enabling users to find and explore DAOs across the ecosystem
- documents_storage: Append-only storage for DAO-related documents with built-in traceability
- dao_sogc_publication: Maintains compliance records from the Swiss Official Gazette of Commerce
- network_call: Secure HTTPS outcall functionality for off-chain interactions
- dao_platform: Frontend hosting directly from canisters for complete decentralization
- internet_identity: WebAuthn-based authentication without passwords or extensions

Each canister operates independently while communicating through well-defined interfaces, creating a resilient system that can scale functionality and regulatory coverage without compromising decentralization. The modular design allows seamless integration of new legal forms, jurisdictions, and features as the platform evolves.

<!-- GETTING STARTED -->

## Getting Started

To get started with dao-forge project, please refer to our setup guide and installation instructions in the documentation section. We provide step-by-step guidelines to help you run the project effectively.

### Prerequisites

Before diving into the dao-forge project, ensure you have the necessary environment and tools set up as listed below.

- [Dfx][dfx-install-url]
- [Node.js][nodejs-install-url]
- [Cargo][cargo-install-url]

### Installation

To install the dao-forge project, follow the steps below.

```bash
# Clone the repository
git clone https://github.com/lorenzoronzani/dao-forge.git

# Install typescript dependencies
npm install
```

### Usage

To run the dao-forge project locally, follow the steps below.

```bash
# Start the replica
dfx start --clean --background

# Deploy the canisters
npm run deploy
```

> Note: If you want to connect to the frontend hosted directly from your machine instead of the one hosted by icp you can run `npm run start`.

### Tests

To run the backend tests, follow the steps below.

```bash
# Build canisters for integration-tests
cargo build

# Run tests
cargo test
```

<!-- ROADMAP -->

## Roadmap

See the [open issues](https://github.com/lorenzoronzani/dao-forge/issues) for a full
list of proposed features (and known issues).

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- LICENSE -->

## License

Distributed under the MIT License. See `LICENSE` for more information.

<!-- CONTACT -->

## Contact

- [Lorenzo Ronzani](https://www.linkedin.com/in/lorenzo-ronzani-658311186/) - lorenzo.ronzani00@gmail.com
- [Giuliano Gremlich](https://www.linkedin.com/in/giuliano-gremlich-265018153/) - giuliano.gremlich@supsi.ch
- [Roberto Guidi](https://www.linkedin.com/in/rguidi/) - roberto.guidi@supsi.ch

Project link: [dao-forge](https://github.com/lorenzoronzani/dao-forge)

<!-- MARKDOWN LINKS & IMAGES -->
[contributors-shield]: https://img.shields.io/github/contributors/lorenzoronzani/dao-forge.svg?style=for-the-badge
[contributors-url]: https://github.com/lorenzoronzani/dao-forge/graphs/contributors

[forks-shield]: https://img.shields.io/github/forks/lorenzoronzani/dao-forge.svg?style=for-the-badge
[forks-url]: https://github.com/lorenzoronzani/dao-forge/network/members

[stars-shield]: https://img.shields.io/github/stars/lorenzoronzani/dao-forge.svg?style=for-the-badge
[stars-url]: https://github.com/lorenzoronzani/dao-forge/stargazers

[issues-shield]: https://img.shields.io/github/issues/lorenzoronzani/dao-forge.svg?style=for-the-badge
[issues-url]: https://github.com/lorenzoronzani/dao-forge/issues

[license-shield]: https://img.shields.io/github/license/lorenzoronzani/dao-forge.svg?style=for-the-badge
[license-url]: https://github.com/lorenzoronzani/dao-forge/blob/master/LICENSE.txt


[ICP]: https://img.shields.io/badge/-Internet%20Computer-3B00B9?style=flat&logo=internetcomputer&logoColor=white
[ICP-url]: https://internetcomputer.org/

[Rust]: https://img.shields.io/badge/-Rust-000000?style=flat&logo=rust&logoColor=white
[Rust-url]: https://rust-lang.org/

[React]: https://img.shields.io/badge/-React-61DAFB?style=flat&logo=react&logoColor=white
[React-url]: https://react.dev/

[Shadcn]: https://img.shields.io/badge/-shadcn/ui-000000?style=flat&logo=shadcnui&logoColor=white
[Shadcn-url]: https://ui.shadcn.com/

[Typescript]: https://img.shields.io/badge/-TypeScript-3178C6?style=flat&logo=typescript&logoColor=white
[Typescript-url]: https://www.typescriptlang.org/


[dfx-install-url]: https://internetcomputer.org/docs/building-apps/developer-tools/dev-tools-overview#dfx
[nodejs-install-url]: https://nodejs.org/en
[cargo-install-url]: https://doc.rust-lang.org/cargo/getting-started/installation.html

<!--
- Shields: https://gist.github.com/kimjisub/360ea6fc43b82baaf7193175fd12d2f7
>