# =============================================================
# deploy.ps1 - Build + deploy to AWS S3 + CloudFront
# Prerequisites: AWS CLI configured, Terraform applied
# Usage: .\scripts\deploy.ps1 -BucketName "your-bucket" -DistributionId "ABCDEF"
# =============================================================
param(
    [Parameter(Mandatory=$true)]
    [string]$BucketName,

    [Parameter(Mandatory=$true)]
    [string]$DistributionId,

    [string]$Region = "us-east-1"
)

$ErrorActionPreference = "Stop"

# 1. Build
Write-Host "`n[1/3] Building portfolio..." -ForegroundColor Cyan
& "$PSScriptRoot\build.ps1"

# 2. Sync to S3
Write-Host "`n[2/3] Uploading to S3..." -ForegroundColor Cyan
aws s3 sync .\dist "s3://$BucketName" `
  --delete `
  --region $Region `
  --cache-control "public, max-age=31536000, immutable" `
  --exclude "*.html"

# HTML: no-cache so users always get latest
aws s3 sync .\dist "s3://$BucketName" `
  --delete `
  --region $Region `
  --cache-control "no-cache, no-store, must-revalidate" `
  --include "*.html" `
  --exclude "*"

Write-Host "S3 upload complete!" -ForegroundColor Green

# 3. CloudFront invalidation
Write-Host "`n[3/3] Invalidating CloudFront cache..." -ForegroundColor Cyan
$invalidation = aws cloudfront create-invalidation `
  --distribution-id $DistributionId `
  --paths "/*" `
  --output json | ConvertFrom-Json

Write-Host "Invalidation created: $($invalidation.Invalidation.Id)" -ForegroundColor Green

Write-Host "`nDeploy complete!" -ForegroundColor Green
Write-Host "Your site will be live in ~5 minutes at your CloudFront URL." -ForegroundColor Yellow
