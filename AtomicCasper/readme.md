# Casper AtomicSwap

Atomic swap uses contract logic and block time to implement a contract which can be used to trade tokens with another contract on a totally different chain (or network),
so it can be used both in-chain (on the same network, different or same chain (test_net, main_net)) and off_chain (on a totally different technology or chain)

## How to compile and use
clone the project into your system, make sure you have necessary tools installed in order to build the contract and deploy it to the network (casper client), by default the compiled version of the contract
can be found in the AtomicCasper folder, but if you want to build it yourself, make sure you have `make` installed and run this command : 

```bash
make build-contract
```

It will generate the contract.wasm file and copy it into deploy folder. the next step is to deploy it into the network using your secrect key. Enter the command as below : 

```bash
casper-client put-deploy --node-address http://<An Node Address>:7777 --chain-name <CHAINNAME> --secret-key <PATH_TO_YOUR_SECRET_KEY_PEM_FILE>  --session-path deploy/contract.wasm --payment-amount <PAYMENTAMOUNT>
```

Where `node-address` can be fetched from cspr.live or testnet.cspr.live in `Tools -> Connected Peers`, just copy one of them that you have access to (you can ping it) and paste it here (without the http and port part!)

and `CHAINNAME` is either `casper-test` if your testing or `casper` to deploy on main_net

`PAYMENTAMOUNT` is a number in motes (each 10^9 mote is 1 casper), I suggest to put XXX CSPRs (XXX000000000) as the value

## How to use the deployed contract

Work in progress


## Contract type
after deployment of the contract, the `init` entrypoint is called, to build storage variables and assign them values, after that, the owner of the contract can call `initiate` 
entrypoint with given information below, to start the process of AtomicSwap on this contract.

`initiate` entrypoint gets `type::String` argument, which must be one of : `NFT`, `ERC-20`, `Direct`, or `Custom`

It also gets an `hash::String` argument, which is a hash of the secret key that will be used to unlock the contract, the hash must be generated using `XXX` algorithm. Also it gets a `destination::AccountHash` argument, which is the account hash of the other account that will be involved in the swap process.


### **NFT**
if the type is `NFT`, the contract will expect the following arguments in the `initiate` entrypoint (besides the `type`, `hash`, `destination` arguments) :
- `contract_hash` : the hash of the contract that holds the NFT token, your contract must have access to this contract in order to transfer the token to/from it. it will be checked in the contract and if it doesn't have access, the contract will revert the transaction.

### **ERC-20**
if the type is `ERC-20`, the contract will expect the following arguments in the `initiate` entrypoint (besides the `type`, `hash` and `destination` arguments) :
- `contract_hash` : the hash of the contract that holds the ERC-20 tokens, your contract must have access to this contract in order to transfer the token to/from it. it will be checked in the contract and if it doesn't have access, the contract will revert the transaction.

### **Direct**
if the type is `Direct`, the contract will expect the following arguments in the `initiate` entrypoint (besides the `type`, `hash` and `destination` arguments) :
- `amount : U512` : the amount of CSPRs that will be transferred to the other account, it must be in motes (10^9 CSPR = 1 CSPR)

    ***Note*** : Direct type requires the purse of the contract to have enough CSPRs to transfer to the other account, if it doesn't have enough CSPRs, the contract will revert the transaction. You should call the contract using a session code(which is found in session folder), get its purse and transfer the `amount` CSPRs to it, then call the `initiate` entrypoint with the `Direct` type, otherwise the contract will fail.

### **Custom**
if the type is `Custom`, the contract will expect the following arguments in the `initiate` entrypoint (besides the `type`, `hash` and `destination` arguments) :
- `contract_hash` : the hash of the contract that holds your tokens, this contract must have access to your tokens in order to transfer them, the contract with contract hash of `contract_hash` must have an `XXtransferXX` entrypoint, which transfers tokens from the contract to the given account hash.

_**Note : In all 4 states above, you must pass an argument named `timeout:u64` which is the time, contract is useable in milliseconds. After that threshold, using contract's unlock entrypoint will return the tokens to first user!**_

## How to unlock the contract
after the contract is initiated, the other account can call the `unlock` entrypoint with the following arguments :

- `secret:string` : the secret key that will be used to unlock the contract, it must be the same secret key that was used to generate the hash that was used to initiate the contract. **Note that after using `unlock` entrypoint, user's password goes public in `secret` field of storage, and can be used to unlock other contract**