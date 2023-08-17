#!/usr/bin/env bash

subxt codegen --derive Clone --derive Debug --derive PartialEq --derive Eq \
  | sed 's/::[ ]*subxt[ ]*::[ ]*utils[ ]*::[ ]*AccountId32/::subxt::utils::Static<::subxt::ext::sp_core::crypto::AccountId32>/g' \
  | rustfmt --edition=2021 > aleph_zeroo.rs;

diff -y -W 200 --suppress-common-lines aleph_zero.rs aleph-node/aleph-client/src/aleph_zero.rs
diff_exit_code=$?
if [[ ! $diff_exit_code -eq 0 ]]; then
  echo "Current runtime metadata is different than versioned in git!"
  echo "Run subxt codegen command as in $(basename $0) from aleph-client directory and commit to git."
  exit 1
fi
echo "Current runtime metadata and versioned in git matches."
