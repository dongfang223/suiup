// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::Args;

use crate::handle_commands::handle_cmd;

use super::ComponentCommands;

/// List available binaries to install.
#[derive(Args, Debug)]
pub struct Command{
    /// List available binaries name
     /// (e.g. 'sui')
    component:String,
}

impl Command {
    pub async fn exec(&self, github_token: &Option<String>) -> Result<()> {
        handle_cmd(ComponentCommands::List{
                        component: self.component.to_owned(),
                    }, 
            github_token.to_owned()
        )
        .await

    }
}
