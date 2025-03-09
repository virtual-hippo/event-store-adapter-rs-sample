use chrono::{Duration, Utc};
use command_domain::{
    acceptance_criterion::AcceptanceCriterion,
    backlog_item::BacklogItem,
    epic::Epic,
    product_backlog::ProductBacklog,
    sprint::Sprint,
    types::{Priority, Status},
};

fn main() {
    // プロダクトバックログの作成
    let mut product_backlog = ProductBacklog::new(
        "ショッピングアプリ".to_string(),
        "オンラインショッピングアプリケーション".to_string(),
    );

    // エピックの作成
    let mut user_auth_epic = Epic::new(
        "ユーザー認証".to_string(),
        "ユーザー登録、ログイン、ログアウト機能".to_string(),
    );

    // バックログアイテムの作成
    let mut login_item = BacklogItem::new(
        "ログイン機能".to_string(),
        "ユーザーがメールアドレスとパスワードでログインできる".to_string(),
    );
    login_item.set_priority(Priority::High);
    login_item.set_story_points(5);

    // 受け入れ基準の追加
    login_item.add_acceptance_criterion(AcceptanceCriterion::new(
        "有効なメールアドレスとパスワードでログインできる".to_string(),
    ));
    login_item.add_acceptance_criterion(AcceptanceCriterion::new(
        "無効な認証情報ではエラーメッセージが表示される".to_string(),
    ));
    login_item.add_acceptance_criterion(AcceptanceCriterion::new(
        "パスワードを忘れた場合のリンクが機能する".to_string(),
    ));

    // バックログアイテムをエピックに追加
    user_auth_epic.add_backlog_item(login_item.clone());

    // 別のバックログアイテムの作成
    let mut register_item = BacklogItem::new(
        "ユーザー登録機能".to_string(),
        "新規ユーザーが登録できる".to_string(),
    );
    register_item.set_priority(Priority::Medium);
    register_item.set_story_points(8);

    // バックログアイテムをエピックに追加
    user_auth_epic.add_backlog_item(register_item.clone());

    // バックログアイテムをプロダクトバックログに追加
    product_backlog.add_item(login_item);
    product_backlog.add_item(register_item);

    // 商品関連のバックログアイテムを作成
    let mut product_list_item = BacklogItem::new(
        "商品一覧表示".to_string(),
        "ユーザーが商品一覧を閲覧できる".to_string(),
    );
    product_list_item.set_priority(Priority::High);
    product_list_item.set_story_points(3);

    let mut product_detail_item = BacklogItem::new(
        "商品詳細表示".to_string(),
        "ユーザーが商品の詳細情報を閲覧できる".to_string(),
    );
    product_detail_item.set_priority(Priority::Medium);
    product_detail_item.set_story_points(3);

    // バックログアイテムをプロダクトバックログに追加
    product_backlog.add_item(product_list_item);
    product_backlog.add_item(product_detail_item);

    // プロダクトバックログの優先順位付け
    product_backlog.prioritize_items();

    // スプリントの作成
    let now = Utc::now();
    let mut sprint = Sprint::new("スプリント1".to_string(), now, now + Duration::days(14));

    // スプリントにバックログアイテムを追加
    if let Some(item) = product_backlog.items().first().cloned() {
        sprint.add_item(item.clone());

        // ステータスを更新
        let mut item_in_progress = item;
        item_in_progress.update_status(Status::InProgress);

        println!("プロダクトバックログ: {}", product_backlog.name());
        println!("  説明: {}", product_backlog.description());
        println!("  アイテム数: {}", product_backlog.items().len());

        println!("\nスプリント: {}", sprint.name());
        println!("  開始日: {}", sprint.start_date());
        println!("  終了日: {}", sprint.end_date());
        println!("  アイテム数: {}", sprint.items().len());

        println!("\nエピック: {}", user_auth_epic.title());
        println!("  説明: {}", user_auth_epic.description());
        println!("  進捗: {:.1}%", user_auth_epic.progress() * 100.0);
    }
}
