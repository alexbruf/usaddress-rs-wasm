/* tslint:disable */
/* eslint-disable */
/**
* @param {string} address
* @returns {ParseResult}
*/
export function parse(address: string): ParseResult;
export type ParseResult = { data: [string, string][] } | { error: string };

