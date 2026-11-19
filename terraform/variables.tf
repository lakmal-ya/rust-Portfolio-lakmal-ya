variable "aws_region" {
  description = "AWS region to deploy to"
  type        = string
  default     = "us-east-1"
}

variable "bucket_name" {
  description = "Globally unique S3 bucket name for the portfolio"
  type        = string
}

variable "domain_name" {
  description = "Custom domain name (leave empty to use the CloudFront domain)"
  type        = string
  default     = ""
}

variable "attach_custom_domain" {
  description = "Attach domain_name (+ www) to CloudFront. Set true only after the ACM certificate is ISSUED."
  type        = bool
  default     = false
}
