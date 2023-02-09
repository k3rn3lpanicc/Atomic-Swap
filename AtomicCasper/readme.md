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
entrypoint with given information below, to start the process of AtomicSwap on this contract. you need to pass : 
