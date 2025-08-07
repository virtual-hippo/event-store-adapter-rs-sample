terraform {
  required_providers {
    tidbcloud = {
      source = "tidbcloud/tidbcloud"
    }
  }
}

resource "random_password" "rmu_app_password" {
  length           = 16
  special          = true
  override_special = "!#$%&*()-_=+[]{}<>:?"
}

resource "tidbcloud_sql_user" "rmu_app" {
  cluster_id   = var.cluster_id
  user_name    = format("%s.%s", var.cluster_user_prefix, "rmu_app") 
  password     = random_password.rmu_app_password.result
  builtin_role = format("%s.%s", var.cluster_user_prefix, "role_readwrite")
}
