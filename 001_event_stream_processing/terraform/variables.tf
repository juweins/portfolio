/*
        This file contains the variables used in the main.tf file.
*/

// Default location for all resources, change to your preference
variable "location" {
  default     = "westeurope"
  description = "The region where the resources will be created."
}

// Default tags to be applied to all resources
variable "default_tags" {
  type = map(string)
  default = {
    environment = "dev"
    owner       = "juweins"
    project     = "001-exchange"
    description = "Exchange Rust Application"
  }
  description = "The default tags to be applied to all resources where possible."
}