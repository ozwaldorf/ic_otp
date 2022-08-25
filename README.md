# Simple OTP Authenticator

A multisig OTP authenticator on the IC. Allows multiple custodians, and supports HOTP and TOTP, loaded through a otpauth URI.

> This is a canister playground for experimenting with the IC. It provides a base using the ic-kit alpha 0.5 release


## Getting Started
### Deploy the canister

The canister can be deployed locally or on the ic mainnet.

> Note: For `local`, it will automatically start the local replica and deploy, or redeploy the canister on the already running replica.

```
make local

make ic
```

> Note: Your dfx principal id is automatically initialized as the first custodian.

### Registering an account

Accounts are registered using an otpauth URI. This can be as verbose as needed, or left to the defaults like most services (discord shown below) utilize.  

```
dfx canister call otp register_otp '(
    "discord", 
    "otpauth://totp/discord:username#1234?secret=000011112222"
  )'
```

### Getting a OTP code

```
dfx canister call otp get_otp "discord"
```

### Removing an account

```
dfx canister call otp remove_otp "discord"
```

## Developing in the repository

You can manually generate the candid by running

```
make candid
```

> The kit automatically generates a rust test `save_candid` for each canister which will generate a candid file to the path specified. The make script runs these cargo tests to save the candid. Furthermore, it's run automatically for `make local|ic` 

You can also clean your environment, which will stop dfx and clean cargo, using

```
make clean
```
