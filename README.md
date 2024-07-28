# solana-nft

Create on-chain NFT[https://medium.com/@elchuo160/create-your-own-on-chain-nfts-on-solana-with-anchor-and-quicknode-a-step-by-step-guide-2024-c108077013e9]
Solana Dynamic Metadata NFT[https://solana.com/developers/guides/token-extensions/dynamic-meta-data-nft]

#### Summary

With the Token Extension program[https://solana.com/developers/guides/token-extensions/getting-started], you can create NFTs and digital assets using the metadata extensions.

Together with Metadata Pointer extension allow you to put any desired metadata natively on-chain. All within a customizable key-value data store directly on the token's mint account, reducing costs and complexity.

All of the NFT attribute fields(name, symbol and uri) are saved using the metadata extension which is pointed to the NFT's mint account, making them accessible to anyone or any program.

#### Metadata Pointer extension

[https://solana.com/developers/guides/token-extensions/metadata-pointer]
Before the Token Extensions Program and the Token Metadata Interface, the process of adding extra data to a Mint Account required creating a Metadata Account through the Metaplex Metadata Program[https://developers.metaplex.com/token-metadata].

The MetadataPointer extension now enables a Mint Account to specify the address of its corresponding Metadata Account. This flexibility allows the Mint Account to point to any account owned by a program that implements the Token Metadata Interface.

The Token Extensions Program directly implements the Token Metadata Interface, made accessible through the TokenMetadata extension. With the TokenMetadata extension, the Mint Account itself can now store the metadata.

#### Minting the NFT

In order to create the NFT we need to perform a following steps:

    Create a mint account
    Initialize the mint account
    Create a metadata pointer account
    Initialize the metadata pointer account
    Create the metadata account
    Initialize the metadata account
    Create the associated token account
    Mint the token to the associated token account
    Freeze the mint authority
