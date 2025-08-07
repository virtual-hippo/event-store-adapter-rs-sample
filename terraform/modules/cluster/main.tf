terraform {
  required_providers {
    tidbcloud = {
      source = "tidbcloud/tidbcloud"
    }
  }
}

resource "tidbcloud_serverless_cluster" "event_store_adapter_rs_sample" {
  project_id = var.project_id
  display_name = "event-store-adapter-rs-sample"
  region = {
    name = "regions/aws-ap-northeast-1"
  }
}
