module github.com/gyuho/infra/aws/go

go 1.21.3

replace (
	github.com/gyuho/infra/go => ../../go
	github.com/gyuho/infra/linux/go => ../../linux/go
)

require (
	github.com/aws/aws-sdk-go-v2 v1.23.5
	github.com/aws/aws-sdk-go-v2/config v1.25.5
	github.com/aws/aws-sdk-go-v2/credentials v1.16.4
	github.com/aws/aws-sdk-go-v2/service/cloudformation v1.40.1
	github.com/aws/aws-sdk-go-v2/service/ec2 v1.137.1
	github.com/aws/aws-sdk-go-v2/service/kms v1.26.3
	github.com/aws/aws-sdk-go-v2/service/s3 v1.44.0
	github.com/aws/aws-sdk-go-v2/service/ssm v1.44.2
	github.com/aws/aws-sdk-go-v2/service/sts v1.25.4
	github.com/dustin/go-humanize v1.0.1
	github.com/gyuho/infra/go v0.0.0-00010101000000-000000000000
	github.com/gyuho/infra/linux/go v0.0.0-00010101000000-000000000000
	github.com/olekukonko/tablewriter v0.0.5
	github.com/spf13/cobra v1.8.0
)

require (
	github.com/aws/aws-sdk-go-v2/aws/protocol/eventstream v1.5.1 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.14.5 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.2.8 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.5.8 // indirect
	github.com/aws/aws-sdk-go-v2/internal/ini v1.7.1 // indirect
	github.com/aws/aws-sdk-go-v2/internal/v4a v1.2.4 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding v1.10.1 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/checksum v1.2.4 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.10.4 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/s3shared v1.16.4 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.17.3 // indirect
	github.com/aws/aws-sdk-go-v2/service/ssooidc v1.20.1 // indirect
	github.com/aws/smithy-go v1.18.1 // indirect
	github.com/google/go-cmp v0.6.0 // indirect
	github.com/inconshreveable/mousetrap v1.1.0 // indirect
	github.com/jmespath/go-jmespath v0.4.0 // indirect
	github.com/mattn/go-runewidth v0.0.15 // indirect
	github.com/rivo/uniseg v0.4.4 // indirect
	github.com/spf13/pflag v1.0.5 // indirect
	go.uber.org/multierr v1.11.0 // indirect
	go.uber.org/zap v1.26.0 // indirect
	k8s.io/utils v0.0.0-20231121161247-cf03d44ff3cf // indirect
)
