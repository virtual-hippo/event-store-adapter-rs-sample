terraform {
  required_providers {
    tidbcloud = {
      source  = "tidbcloud/tidbcloud"
      version = "~> 0.4.3"
    }
  }
  required_version = "= 1.12.2"
}

provider "tidbcloud" {
  # 環境変数から自動で読み込み
  sync        = true
}

# プロジェクト参照
data "tidbcloud_projects" "default_project" {
  page      = 1
  page_size = 10
}

module "cluster" {
  source = "./modules/cluster"
  project_id = data.tidbcloud_projects.default_project.items[0].id
}

module "sql_user" {
  source = "./modules/sql-user"
  cluster_id = module.cluster.cluster_id
  cluster_user_prefix = module.cluster.user_prefix
}