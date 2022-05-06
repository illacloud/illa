// Copyright 2022 The ILLA Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package root

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

func NewCmdRoot() (cmd *cobra.Command) {
	cmd = &cobra.Command{
		Use:   "illa <command> [options]",
		Short: "ILLA CLI",
		Long:  `Deploy a modern low-code platform in 5 Seconds!`,
	}
	return cmd
}

func Execute() {
	rootCmd := NewCmdRoot()
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
