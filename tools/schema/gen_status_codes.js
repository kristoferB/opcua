let _ = require("lodash");
let csv = require("csv-parser");
let fs = require("fs");

let settings = require("./settings");

let status_code_csv = `${settings.schema_dir}/Opc.Ua.StatusCodes.csv`;

let rs_out = fs.createWriteStream(`${settings.rs_status_codes_dir}/status_codes.rs`);

let status_codes = [
    {
        var_name: "Good",
        str_code: "Good",
        hex_code: 0,
        description: "Good"
    }
];


fs.createReadStream(status_code_csv)
    .pipe(csv(['str_code', 'hex_code', 'description']))
    .on('data', data => {
        data.var_name = data.str_code;
        status_codes.push(data);
    })
    .on('end', () => {
        // Sort status
        status_codes = _.sortBy(status_codes, ['hex_code']);

        rs_out.write(
            `// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock

// This file was autogenerated from Opc.Ua.StatusCodes.csv by tools/schema/gen_status_codes.js
// DO NOT EDIT THIS FILE
#![allow(non_upper_case_globals)]
#![allow(unused_attributes)]

use std::str::FromStr;

// This file contains the machine-generated values of StatusCode. Refer to status_code.rs for the hand implemented
// components.

bitflags! {
    pub struct StatusCode: u32 {
        // The UPPERCASE values are bitflags. The PascalCase values are OPC UA Status codes.

        // Mask for the status code section
        const STATUS_MASK = 0xffff_0000;
        // Mask for the bits section
        const BIT_MASK = 0x0000_ffff;

        // Flag for an error / uncertain code
        const IS_ERROR                = 0x8000_0000;
        const IS_UNCERTAIN            = 0x4000_0000;

        // Historian bits 0:4
        const HISTORICAL_RAW          = 0x0000_0000;
        const HISTORICAL_CALCULATED   = 0x0000_0001;
        const HISTORICAL_INTERPOLATED = 0x0000_0002;
        const HISTORICAL_RESERVED     = 0x0000_0003;
        const HISTORICAL_PARTIAL      = 0x0000_0004;
        const HISTORICAL_EXTRA_DATA   = 0x0000_0008;
        const HISTORICAL_MULTI_VALUE  = 0x0000_0010;
        // Overflow bit 7
        const OVERFLOW                = 0x0000_0080;
        // Limit bits 8:9
        const LIMIT_LOW               = 0x0000_0100;
        const LIMIT_HIGH              = 0x0000_0200;
        const LIMIT_CONSTANT          = 0x0000_0300;
        // Info type bits 10:11
        const LIMIT_DATA_VALUE        = 0x0000_2000;
        // Semantics changed bit 14
        const SEMANTICS_CHANGED       = 0x0000_4000;
        // Semantics changed bit 15
        const STRUCTURE_CHANGED       = 0x0000_8000;

        // Actual status codes follow here
`);
        _.each(status_codes, data => {
            rs_out.write(`        const ${data.var_name} = ${hexcode("" + data.hex_code)};\n`);
        });
        rs_out.write(
            `    }
}

impl StatusCode {
`);

        rs_out.write(`
    /// Returns the descriptive name for the status code, e.g. to put a meaningful code in a log file
    pub fn name(self) -> &'static str {
        match self.status() {
`);
        _.each(status_codes, data => {
            rs_out.write(`            StatusCode::${data.var_name} => "${data.str_code}",\n`);
        });
        rs_out.write(`            _ => "Unrecognized status code",
        }
    }
`);

        rs_out.write(`
    /// Returns the descriptive text for the status code
    pub fn description(self) -> &'static str {
        match self.status() {
`);
        _.each(status_codes, data => {
            rs_out.write(`            StatusCode::${data.var_name} => "${data.description}",\n`);
        });
        rs_out.write(`            _ => "Unrecognized status code",
        }
    }
`);

        rs_out.write(`
    /// Takes an OPC UA status code as a UInt32 and returns the matching StatusCode, assuming there is one
    /// Note that this is lossy since any bits associated with the status code will be ignored.
    pub fn from_u32(code: u32) -> Option<StatusCode> {
        StatusCode::from_bits(code)
    }
}
`);

        rs_out.write(`

impl FromStr for StatusCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
`);
        _.each(status_codes, data => {
            rs_out.write(`            "${data.str_code}" => Ok(StatusCode::${data.var_name}),\n`);
        });
        rs_out.write(`            _ => Err(())
        }
    }
}

`);

        rs_out.write(``);

    });

function hexcode(v) {
    if (v.length == 10) {
        // Hexcode is 0xNNNNNNNN, returned as 0xNNNN_NNNN
        return v.slice(0, 6) + "_" + v.slice(6);
    } else {
        return v;
    }
}
