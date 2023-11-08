# SPDX-FileCopyrightText: 2023 Kamila Borowska <kamila@borowska.pw>
#
# SPDX-License-Identifier: AGPL-3.0-or-later

{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs =
    [ cargo clippy diesel-cli nodejs-16_x openssl pkg-config postgresql.lib rustfmt ];
}
