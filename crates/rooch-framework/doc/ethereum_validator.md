
<a name="0x3_ethereum_validator"></a>

# Module `0x3::ethereum_validator`

This module implements Ethereum validator with the ECDSA recoverable signature over Secp256k1.


-  [Struct `EthereumValidator`](#0x3_ethereum_validator_EthereumValidator)
-  [Constants](#@Constants_0)
-  [Function `auth_validator_id`](#0x3_ethereum_validator_auth_validator_id)
-  [Function `rotate_authentication_key_entry`](#0x3_ethereum_validator_rotate_authentication_key_entry)
-  [Function `remove_authentication_key_entry`](#0x3_ethereum_validator_remove_authentication_key_entry)
-  [Function `public_key_to_address`](#0x3_ethereum_validator_public_key_to_address)
-  [Function `public_key_to_authentication_key`](#0x3_ethereum_validator_public_key_to_authentication_key)
-  [Function `get_authentication_key_option_from_account`](#0x3_ethereum_validator_get_authentication_key_option_from_account)
-  [Function `is_authentication_key_in_account`](#0x3_ethereum_validator_is_authentication_key_in_account)
-  [Function `get_authentication_key_from_account`](#0x3_ethereum_validator_get_authentication_key_from_account)
-  [Function `validate_signature`](#0x3_ethereum_validator_validate_signature)
-  [Function `validate`](#0x3_ethereum_validator_validate)


<pre><code><b>use</b> <a href="">0x1::debug</a>;
<b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="">0x1::signer</a>;
<b>use</b> <a href="">0x1::vector</a>;
<b>use</b> <a href="">0x2::context</a>;
<b>use</b> <a href="">0x2::hex</a>;
<b>use</b> <a href="account_authentication.md#0x3_account_authentication">0x3::account_authentication</a>;
<b>use</b> <a href="auth_payload.md#0x3_auth_payload">0x3::auth_payload</a>;
<b>use</b> <a href="auth_validator.md#0x3_auth_validator">0x3::auth_validator</a>;
<b>use</b> <a href="ecdsa_k1.md#0x3_ecdsa_k1">0x3::ecdsa_k1</a>;
<b>use</b> <a href="ethereum_address.md#0x3_ethereum_address">0x3::ethereum_address</a>;
</code></pre>



<a name="0x3_ethereum_validator_EthereumValidator"></a>

## Struct `EthereumValidator`



<pre><code><b>struct</b> <a href="ethereum_validator.md#0x3_ethereum_validator_EthereumValidator">EthereumValidator</a> <b>has</b> drop, store
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x3_ethereum_validator_ErrorInvalidPublicKeyLength"></a>



<pre><code><b>const</b> <a href="ethereum_validator.md#0x3_ethereum_validator_ErrorInvalidPublicKeyLength">ErrorInvalidPublicKeyLength</a>: u64 = 1;
</code></pre>



<a name="0x3_ethereum_validator_ETHEREUM_AUTH_VALIDATOR_ID"></a>

there defines auth validator id for each blockchain


<pre><code><b>const</b> <a href="ethereum_validator.md#0x3_ethereum_validator_ETHEREUM_AUTH_VALIDATOR_ID">ETHEREUM_AUTH_VALIDATOR_ID</a>: u64 = 1;
</code></pre>



<a name="0x3_ethereum_validator_auth_validator_id"></a>

## Function `auth_validator_id`



<pre><code><b>public</b> <b>fun</b> <a href="ethereum_validator.md#0x3_ethereum_validator_auth_validator_id">auth_validator_id</a>(): u64
</code></pre>



<a name="0x3_ethereum_validator_rotate_authentication_key_entry"></a>

## Function `rotate_authentication_key_entry`



<pre><code><b>public</b> entry <b>fun</b> <a href="ethereum_validator.md#0x3_ethereum_validator_rotate_authentication_key_entry">rotate_authentication_key_entry</a>(ctx: &<b>mut</b> <a href="_Context">context::Context</a>, <a href="account.md#0x3_account">account</a>: &<a href="">signer</a>, public_key: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<a name="0x3_ethereum_validator_remove_authentication_key_entry"></a>

## Function `remove_authentication_key_entry`



<pre><code><b>public</b> entry <b>fun</b> <a href="ethereum_validator.md#0x3_ethereum_validator_remove_authentication_key_entry">remove_authentication_key_entry</a>(ctx: &<b>mut</b> <a href="_Context">context::Context</a>, <a href="account.md#0x3_account">account</a>: &<a href="">signer</a>)
</code></pre>



<a name="0x3_ethereum_validator_public_key_to_address"></a>

## Function `public_key_to_address`

TODO: fix this
remove get_public_key_from_authenticator_payload function
authenticator_payload is no public key included, Should use ecdsa k1 ecrecover from sign

Get the authentication key of the given authenticator from authenticator_payload.


<pre><code><b>public</b> <b>fun</b> <a href="ethereum_validator.md#0x3_ethereum_validator_public_key_to_address">public_key_to_address</a>(public_key: <a href="">vector</a>&lt;u8&gt;): <a href="ethereum_address.md#0x3_ethereum_address_ETHAddress">ethereum_address::ETHAddress</a>
</code></pre>



<a name="0x3_ethereum_validator_public_key_to_authentication_key"></a>

## Function `public_key_to_authentication_key`

Get the authentication key of the given public key.


<pre><code><b>public</b> <b>fun</b> <a href="ethereum_validator.md#0x3_ethereum_validator_public_key_to_authentication_key">public_key_to_authentication_key</a>(public_key: <a href="">vector</a>&lt;u8&gt;): <a href="">vector</a>&lt;u8&gt;
</code></pre>



<a name="0x3_ethereum_validator_get_authentication_key_option_from_account"></a>

## Function `get_authentication_key_option_from_account`

Get the authentication key option of the given account.


<pre><code><b>public</b> <b>fun</b> <a href="ethereum_validator.md#0x3_ethereum_validator_get_authentication_key_option_from_account">get_authentication_key_option_from_account</a>(ctx: &<a href="_Context">context::Context</a>, addr: <b>address</b>): <a href="_Option">option::Option</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;
</code></pre>



<a name="0x3_ethereum_validator_is_authentication_key_in_account"></a>

## Function `is_authentication_key_in_account`

The authentication key exists in account or not.


<pre><code><b>public</b> <b>fun</b> <a href="ethereum_validator.md#0x3_ethereum_validator_is_authentication_key_in_account">is_authentication_key_in_account</a>(ctx: &<a href="_Context">context::Context</a>, addr: <b>address</b>): bool
</code></pre>



<a name="0x3_ethereum_validator_get_authentication_key_from_account"></a>

## Function `get_authentication_key_from_account`

Extract the authentication key of the authentication key option.


<pre><code><b>public</b> <b>fun</b> <a href="ethereum_validator.md#0x3_ethereum_validator_get_authentication_key_from_account">get_authentication_key_from_account</a>(ctx: &<a href="_Context">context::Context</a>, addr: <b>address</b>): <a href="">vector</a>&lt;u8&gt;
</code></pre>



<a name="0x3_ethereum_validator_validate_signature"></a>

## Function `validate_signature`

Only validate the authenticator's signature.


<pre><code><b>public</b> <b>fun</b> <a href="ethereum_validator.md#0x3_ethereum_validator_validate_signature">validate_signature</a>(payload: <a href="auth_payload.md#0x3_auth_payload_AuthPayload">auth_payload::AuthPayload</a>, tx_hash: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<a name="0x3_ethereum_validator_validate"></a>

## Function `validate`



<pre><code><b>public</b> <b>fun</b> <a href="ethereum_validator.md#0x3_ethereum_validator_validate">validate</a>(ctx: &<a href="_Context">context::Context</a>, authenticator_payload: <a href="">vector</a>&lt;u8&gt;)
</code></pre>
