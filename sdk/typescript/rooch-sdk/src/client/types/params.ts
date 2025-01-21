// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

/**
 *  ######################################
 *  ### DO NOT EDIT THIS FILE DIRECTLY ###
 *  ######################################
 *
 * This file is generated from:
 * /crates/rooch-open-rpc-spec/openrpc.json
 */

import type * as RpcTypes from './generated.js'
import type { address } from '../../types/index.js'
/** Broadcast a Bitcoin transaction */
export interface BroadcastTXParams {
  hex: string
  maxfeerate?: number | null | undefined
  maxburnamount?: number | null | undefined
}
/** Query the Inscription via global index by Inscription filter */
export interface QueryInscriptionsParams {
  filter: RpcTypes.InscriptionFilterView
  cursor?: RpcTypes.IndexerStateIDView | null | undefined
  limit?: string | null | undefined
  descendingOrder?: boolean | null | undefined
}
/** Query the UTXO via global index by UTXO filter */
export interface QueryUTXOsParams {
  filter: RpcTypes.UTXOFilterView
  cursor?: RpcTypes.IndexerStateIDView | null | undefined
  limit?: string | null | undefined
  descendingOrder?: boolean | null | undefined
}
export interface DryRunRawTransactionParams {
  txBcsHex: string
}
/**
 * Send the signed transaction in bcs hex format This method blocks waiting for the transaction to be
 * executed.
 */
export interface ExecuteRawTransactionParams {
  txBcsHex: string
  txOption?: RpcTypes.TxOptions | null | undefined
}
/** Execute a read-only function call The function do not change the state of Application */
export interface ExecuteViewFunctionParams {
  functionCall: RpcTypes.FunctionCallView
}
/** get account balance by RoochAddress and CoinType */
export interface GetBalanceParams {
  owner: address
  coinType: string
}
/** get account balances by RoochAddress */
export interface GetBalancesParams {
  owner: address
  cursor?: RpcTypes.IndexerStateIDView | null | undefined
  limit?: string | null | undefined
}
export interface GetChainIDParams {}
/** Get the events by event handle id */
export interface GetEventsByEventHandleParams {
  eventHandleType: string
  cursor?: string | null | undefined
  limit?: string | null | undefined
  descendingOrder?: boolean | null | undefined
  eventOptions?: RpcTypes.EventOptions | null | undefined
}
/** Get Object Fields via ObjectID and field keys. */
export interface GetFieldStatesParams {
  objectId: string
  fieldKey: string[]
  stateOption?: RpcTypes.StateOptions | null | undefined
}
/** get module ABI by module id */
export interface GetModuleABIParams {
  moduleAddr: string
  moduleName: string
}
/** Get object states by object id */
export interface GetObjectStatesParams {
  objectIds: string
  stateOption?: RpcTypes.StateOptions | null | undefined
}
/**
 * Get the states by access_path If the StateOptions.decode is true, the state is decoded and the
 * decoded value is returned in the response.
 */
export interface GetStatesParams {
  accessPath: string
  stateOption?: RpcTypes.StateOptions | null | undefined
}
export interface GetTransactionsByHashParams {
  txHashes: string[]
}
export interface GetTransactionsByOrderParams {
  cursor?: string | null | undefined
  limit?: string | null | undefined
  descendingOrder?: boolean | null | undefined
}
/** List Object Fields via ObjectID. */
export interface ListFieldStatesParams {
  objectId: string
  cursor?: string | null | undefined
  limit?: string | null | undefined
  stateOption?: RpcTypes.StateOptions | null | undefined
}
/**
 * List the states by access_path If the StateOptions.decode is true, the state is decoded and the
 * decoded value is returned in the response.
 */
export interface ListStatesParams {
  accessPath: string
  cursor?: string | null | undefined
  limit?: string | null | undefined
  stateOption?: RpcTypes.StateOptions | null | undefined
}
/** Query the events indexer by event filter */
export interface QueryEventsParams {
  filter: RpcTypes.EventFilterView
  cursor?: RpcTypes.IndexerEventIDView | null | undefined
  limit?: string | null | undefined
  queryOption?: RpcTypes.QueryOptions | null | undefined
}
/** Query the object states indexer by state filter */
export interface QueryObjectStatesParams {
  filter: RpcTypes.ObjectStateFilterView
  cursor?: RpcTypes.IndexerStateIDView | null | undefined
  limit?: string | null | undefined
  queryOption?: RpcTypes.QueryOptions | null | undefined
}
/** Query the transactions indexer by transaction filter */
export interface QueryTransactionsParams {
  filter: RpcTypes.TransactionFilterView
  cursor?: string | null | undefined
  limit?: string | null | undefined
  queryOption?: RpcTypes.QueryOptions | null | undefined
}
/** Repair indexer by sync from states */
export interface RepairIndexerParams {
  repairType: string
  repairParams: RpcTypes.RepairIndexerParamsView
}
/**
 * Send the signed transaction in bcs hex format This method does not block waiting for the transaction
 * to be executed.
 */
export interface SendRawTransactionParams {
  txBcsHex: string
}
/** Get the chain and service status */
export interface StatusParams {}
/** Sync state change sets */
export interface SyncStatesParams {
  filter: RpcTypes.SyncStateFilterView
  cursor?: string | null | undefined
  limit?: string | null | undefined
  queryOption?: RpcTypes.QueryOptions | null | undefined
}
