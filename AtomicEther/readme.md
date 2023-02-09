# Casper AtomicSwap

Atomic swap uses contract logic and block time to implement a contract which can be used to trade tokens with another contract on a totally different chain (or network),
so it can be used both in-chain (on the same network, different or same chain (test_net, main_net)) and off_chain (on a totally different technology or chain)

## How to compile and use
clone the project into your system, make sure you have necessary tools installed in order to build the contract and deploy it to the network (casper client), by default the compiled version of the contract
can be found in the AtomicCasper folder, but if you want to build it yourself, make sure you have `make` installed and run this command : 

```bash
Command to build the contract Coming Soon...
```

It will generate the contract file and copy it into deploy folder. the next step is to deploy it into the network using your secrect key. Enter the command as below : 

```bash
Deploy Command Coming Soon...
```

Where : Coming Soon

## How to use the deployed contract

Work in progress


## Contract type

In Progress, Coming Soon

`NFT`, `ERC-20`, `Direct`, or `Custom`
`hash`, `destination`, `timeout`

### **NFT**
Coming Soon...

if the type is `NFT`, the contract will expect 
 - `contract_hash` : the hash of the contract that holds the NFT token, your contract must have access to this contract in order to transfer the token to/from it. it will be checked in the contract and if it doesn't have access, the contract will revert the transaction.

### **ERC-20**
Coming Soon...
- `contract_hash`

### **Direct**
- `amount`

### **Custom**
- `contract_hash`

## How to unlock the contract
after the contract is initiated, the other account can call the `unlock` entrypoint with the following arguments :

- `secret:string` : the secret key that will be used to unlock the contract, it must be the same secret key that was used to generate the hash that was used to initiate the contract. **Note that after using `unlock` entrypoint, user's password goes public in `secret` field of storage, and can be used to unlock other contract**