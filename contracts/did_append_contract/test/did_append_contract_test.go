// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package test

import (
	"testing"

	"github.com/stretchr/testify/require"

	"???t/d/Master2/PFE/contracts/DID_append/src/did_append_contract/go/did_append_contract"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, did_append_contract.ScName, did_append_contract.OnDispatch)
	require.NoError(t, ctx.ContractExists(did_append_contract.ScName))
}
