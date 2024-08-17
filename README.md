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

#### Mint Account

The mint account of a token contains the following attributes(fields):

- Owner: Token Program. automatically set
- Mint Authority: Edition
- Freeze Authority: Edition
- Supply: to set it to 1 because it is a NFT
- Decimals: 0 because it is a NFT

#### Token Account

The tokne account of a token contains the following attributes(fields):

- mint: The mint address associated with this account.
- owner: The address that has authority over this account.
- amount: The number of tokens in this account.
- delegate: Address of another address that can manage your token account. That means, transferring or freezing your asset.
- state: The account state. It is an enum of three possible values, Uninitialized, for when the account does not exist, Initialized, for when the account has been created and exists, Frozen, for when the account has been frozen by the freeze authority.
- is_native: is this token the native Solana token.
- delegated_amount: amount delegated to the delegate field mentioned above.
- close_authority: Address which can close this account.

#### Associated Token Account(ATA)

oken account to map to the user's wallet, using the Associated Token Account.

The Associated Token Account is a PDA that is deterministically derived using the address and mint account.

#### Metadata Account

https://developers.metaplex.com/token-metadata

- Owner: Token Metadata Program
- key: MetadataV1, an enum that lets Metaplex identify the 'type of metaplex account',
- Update Authority
- Mint
- Name
- Data:
- Symbol
- URI
- Seller Fee Basis Point
- Creators
- Primary Sale Happened
- Is Mutable
- Edition Nonce
- Token Standard: optional enum= NonFungible(Master Edition), FungibleAsset(supply > 1 with attributes) , Fungible, ProgrammableNonFungible
- Collection: the collectionNft public key or None
- Uses: optioanl, to be deprecated
- Collection Details
- Programmable Configs

These markers and accounts

- MetadataV1: This account holds the token metadata(the one we're currently working with).
- MasterEditionV1 and MasterEditionV2: The master edition account allows NFTs to be printed a limited or unlimited times amount of times. When we say printed, what we mean is making copies of the NFT
- EditionV1: The edition account derived from a mint account represents an NFT that was copied from a Master Edition NFT.
- EditionMarker and EditionMarkerV2: This account is used to internally keep track of which editions were printed and which ones have not.
- TokenRecord: used by programmable NFTs only. Token Record accounts enable us to attach custom data to token accounts rather than mint account.
- MetadataDelegate: These accounts are used to store multiple delegate authorities for a given metadata account.
- CollectionAuthorityRecord: Keeps track of which authorities are allowed to set and/or verify the collection of the metadata account.
- UseAuthorityRecord: Keeps track of which authorities are allowed to reduce the uses(might be deprecated soon) field on the metadata account.
- TokenOwnedEscrow: An escrow account that is managed by the holder of the NFT.

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
