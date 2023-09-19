/*
        This is the main config file to create the outlined infrastructure.
        It starts from scratch. (Greenfield)

        In order to run the app, the following resources need to be created:
        - Resource Group
        - Storage Account
        - (App) Service Plan
        - App Insights
        - Azure Function

    !!!!  ATTENTION: These resources are not free. You may get charged. !!!!
*/

// This is a random suffix to make sure the names are unique
resource "random_id" "random-suffix" {
  byte_length = 6
}

// This is the resource group for all resources
resource "azurerm_resource_group" "baserg" {
  name     = "exchange-rg"
  location = var.location
  tags     = var.default_tags
}

// This uses the inexpensive (LRS) of Azure Storage (available in westeurope)
resource "azurerm_storage_account" "basestorage" {
  name                     = "exchangestorage-${random_id.random-suffix}"
  resource_group_name      = azurerm_resource_group.baserg.name
  location                 = var.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
  tags                     = var.default_tags
}

// This uses the free tier (F1) of Azure Functions (available in westeurope)
resource "azurerm_service_plan" "baseplan" {
  name                = "SP-exchange-plan-${random_id.random-suffix}"
  location            = var.location
  resource_group_name = azurerm_resource_group.baserg.name
  os_type             = "Linux"
  sku_name            = "F1"
  tags                = var.default_tags
}

// TODO: Change to match the app requirements
resource "azurerm_application_insights" "baseinsight" {
  name                = "ai-exchange-${random_id.random-suffix}"
  location            = azurerm_resource_group.baserg.location
  resource_group_name = azurerm_resource_group.baserg.name
  application_type    = "web"
  tags                = var.default_tags
}

// TODO: Change to match the app requirements
resource "azurerm_linux_function_app" "exchange" {
  name                = "exchange-func-${random_id.random-suffix}"
  resource_group_name = azurerm_resource_group.baserg.name
  location            = azurerm_resource_group.baserg.location

  storage_account_name       = azurerm_storage_account.basestorage.name
  storage_account_access_key = azurerm_storage_account.basestorage.primary_access_key
  service_plan_id            = azurerm_service_plan.baseplan.id
  tags                       = var.default_tags

  site_config {}
}