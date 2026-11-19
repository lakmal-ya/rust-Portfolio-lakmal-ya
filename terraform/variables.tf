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

variable "attach_aliases" {
  description = "Set false to attach the certificate without the aliases (used when moving the domain from another distribution)"
  type        = bool
  default     = true
}

variable "attach_apex" {
  description = "Attach the apex domain as well as www. Set false while the apex is still held by another CloudFront distribution."
  type        = bool
  default     = true
}
