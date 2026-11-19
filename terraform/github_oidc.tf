# ============================================================
# GitHub Actions OIDC -> IAM role for CI deploys (no static keys)
# ============================================================

variable "github_repo" {
  description = "GitHub repo (owner/name) allowed to assume the deploy role"
  type        = string
  default     = "lakmal-ya/rust-Portfolio-lakmal-ya"
}

variable "github_branch" {
  description = "Branch allowed to deploy"
  type        = string
  default     = "main"
}

# The account already has the GitHub OIDC provider (shared with other projects),
# so reference it instead of creating a second one.
data "aws_iam_openid_connect_provider" "github" {
  url = "https://token.actions.githubusercontent.com"
}

data "aws_iam_policy_document" "github_trust" {
  statement {
    effect  = "Allow"
    actions = ["sts:AssumeRoleWithWebIdentity"]

    principals {
      type        = "Federated"
      identifiers = [data.aws_iam_openid_connect_provider.github.arn]
    }

    condition {
      test     = "StringEquals"
      variable = "token.actions.githubusercontent.com:aud"
      values   = ["sts.amazonaws.com"]
    }

    condition {
      test     = "StringEquals"
      variable = "token.actions.githubusercontent.com:sub"
      values   = ["repo:${var.github_repo}:ref:refs/heads/${var.github_branch}"]
    }
  }
}

resource "aws_iam_role" "github_deploy" {
  name                 = "portfolio-github-deploy"
  description          = "Assumed by GitHub Actions (${var.github_repo}@${var.github_branch}) to deploy the portfolio"
  assume_role_policy   = data.aws_iam_policy_document.github_trust.json
  max_session_duration = 3600

  tags = {
    ManagedBy = "Terraform"
  }
}

data "aws_iam_policy_document" "github_deploy" {
  statement {
    sid       = "ListBucket"
    actions   = ["s3:ListBucket"]
    resources = [aws_s3_bucket.portfolio.arn]
  }

  statement {
    sid       = "WriteObjects"
    actions   = ["s3:GetObject", "s3:PutObject", "s3:DeleteObject"]
    resources = ["${aws_s3_bucket.portfolio.arn}/*"]
  }

  statement {
    sid       = "InvalidateCache"
    actions   = ["cloudfront:CreateInvalidation", "cloudfront:GetInvalidation"]
    resources = [aws_cloudfront_distribution.portfolio.arn]
  }
}

resource "aws_iam_role_policy" "github_deploy" {
  name   = "portfolio-deploy"
  role   = aws_iam_role.github_deploy.id
  policy = data.aws_iam_policy_document.github_deploy.json
}

output "github_deploy_role_arn" {
  description = "Set as the AWS_ROLE repository variable"
  value       = aws_iam_role.github_deploy.arn
}
