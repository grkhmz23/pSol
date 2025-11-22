# System Architecture Diagram

~~~mermaid
graph TD
    subgraph Client Surfaces
        Wallet[Wallet / dApp]
        CLI[CLI & future SDK]
    end

    subgraph ProtocolLayer[Privacy Protocol (programs/psol)]
        PrivacyProgram[Privacy Program (future encrypted balances)]
    end

    subgraph TokenLayer[pSOL Token (programs/psol-token)]
        Vault[Vault PDA (locks SOL)]
        Mint[pSOL SPL Mint]
        TokenAccounts[pSOL Token Accounts]
    end

    Wallet -->|Swap / Transfer| Mint
    Mint --> TokenAccounts
    Vault --> Mint
    CLI --> PrivacyProgram
~~~ 
