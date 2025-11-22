# Deployment Pipeline Diagram

~~~mermaid
graph LR
    Dev[Developer Commit] --> CI[CI Build]
    CI --> Tests[Anchor Tests]
    Tests --> Artifacts[Program artifacts + IDLs]
    Artifacts --> Devnet[Devnet Deploy]
    Devnet --> Smoke[Smoke Tests]
    Smoke --> Review[Manual Review]
    Review --> Prod[Mainnet-beta Deploy]
~~~ 
