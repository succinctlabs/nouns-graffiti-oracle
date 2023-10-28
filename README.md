# Prop 249 x Succinct: Distributing Solo Staking Rewards with a ZK-powered Autonomous System

![Cover](https://i.imgur.com/s5WMDWq.png)

## **TL;DR**

We propose Nouns deploy an autonomous, trust-minimized solution to [Prop 249: Nouns block graffiti x ETH solo staking](https://nouns.wtf/vote/249) using Succinct’s ZK Light Client. This proposal will also extend the 10 ETH monthly prize pool for the next 3 months, where any solo staker that puts “⌐◨-◨” in their graffiti flag will be eligible to win 1 ETH through a fully autonomous on-chain prize-pool system powered by zero-knowledge proofs.

## Why Nouns should deploy a ZK-powered autonomous solution to Prop 249

**A quick refresher on Prop 249**

[Prop 249](https://nouns.wtf/vote/249) was passed to create a 10ETH monthly prize pool to reward solo stakers who put "⌐◨-◨" (noggles) into their validator's graffiti flag. Solo staking increases the decentralization of the Ethereum network and this prize pool incentivizes solo stakers while proliferating the Nouns meme.

**The original solution**

Originally, Nouns planned to operate the Noggles prize pool via a multisig operated by a few trusted volunteers. Everyone would have to trust the multisig participants with operating the prize pool correctly. If the multisig participants turned malicious they could potentially steal all the funds in the prize pool for themselves! Additionally, operating this multisig placed a burden on these volunteers who had to be operationally responsible for the prize pool distributions over a period of months, as well as worry about secure key management.

**Aspiring to autonomous systems**

In contrast, the most inspiring protocols in crypto are fully on-chain, unstoppable, autonomous systems. Nouns DAO is a great example—all Noun’s governance is conducted fully on-chain. **Autonomous and trustless systems are a core part of the Nouns’ ethos and are very Nounish. These sorts of systems form the foundation of the blockchain substrate and further enabling autonomy and trustlessness with new, frontier technology like zero-knowledge proofs is extremely aligned with Noun’s values.** Can we remove the multisig from Prop 249 and run the Noggles graffiti prize pool as a fully permissionless autonomous system? Enter the Succinct ZK light client…

**Using Succinct’s zkSNARK light client** 

Succinct’s Ethereum zkSNARK light client is not only useful for cross-chain protocols, but is also useful for use-cases involving consensus data in the execution layer! With this realization, we originally won [Nouns Prop House Open Round 20](https://prop.house/nouns/open-round-20/6036) to develop a proof of concept for how Succinct’s ZK light client can be used for verifying Noggles in solo staking validator graffiti. We realized that we could take this POC and build an entirely autonomous system (a system that runs with at least 1 economically rational actor) to help operate the prize pool from Prop 249.

**We see Prop 249 as the perfect opportunity for Nouns to deploy a project utilizing the forefront of exciting new cryptographic primitives.** By demonstrating how zero-knowledge technology can empower the autonomous maintenance of a public good protocol (Noggles prize pool incentivizing Ethereum’s decentralization with solo staking), **Nouns can demonstrate how ZK can enable the future of trustless and expressive dApps.**
