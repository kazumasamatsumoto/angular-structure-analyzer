申し訳ありません。コード生成が途中で切れてしまいました。作成したファイルを整理して再度提示します。

## Angular 構造解析ツール（Rust 製）のファイル構成

```
angular-structure-analyzer/
├── Cargo.toml                     # プロジェクト設定ファイル
└── src/
    ├── main.rs                    # メインエントリーポイント
    ├── models.rs                  # データモデル定義
    ├── analyzer.rs                # 解析ロジック
    ├── utils.rs                   # ユーティリティ関数
    └── formatter/                 # 出力フォーマッタ
        ├── mod.rs                 # モジュール定義
        ├── text.rs                # テキスト出力形式
        ├── json.rs                # JSON出力形式
        └── graph.rs               # グラフ出力形式
```

このツールは、Angular プロジェクトの構造を解析し、コンポーネント、サービス、モジュール、依存関係などの情報を視覚的に表示します。

## 各ファイルの役割

1. **Cargo.toml**

   - プロジェクトの依存関係と設定（clap, walkdir, regex, colored, serde など）

2. **main.rs**

   - コマンドライン引数の解析
   - 各解析コマンドの実行

3. **models.rs**

   - プロジェクト構造を表すデータモデル（コンポーネント、サービス、モジュールなど）

4. **analyzer.rs**

   - Angular プロジェクトのファイル解析
   - TypeScript ファイルの解析（コンポーネント、サービスなどの検出）

5. **utils.rs**

   - 文字列操作ユーティリティ（kebab-case, PascalCase 変換など）
   - ファイルシステム操作ユーティリティ

6. **formatter/**
   - **mod.rs**: フォーマッタモジュールの定義
   - **text.rs**: コンソールへの色付きテキスト出力
   - **json.rs**: JSON 形式での出力
   - **graph.rs**: 依存関係グラフの表示

## ツールの機能

このツールは以下の機能を提供します：

1. **プロジェクト構造の表示**

   - ディレクトリ構造を再帰的に表示
   - ファイルタイプによる色分け表示

2. **コンポーネント解析**

   - すべての Angular コンポーネントを検出
   - セレクタ、テンプレート、スタイルの関連付け

3. **サービス解析**

   - すべての Angular サービスを検出
   - インジェクションスコープの表示

4. **モジュール解析**

   - すべての Angular モジュールを検出
   - 宣言、インポート、エクスポート、プロバイダの表示

5. **依存関係解析**

   - ファイル間の依存関係を検出
   - 依存関係グラフの表示

6. **ルート解析**
   - Angular ルーティング設定の解析
   - ルート階層の表示

## 使用例

```bash
# プロジェクト構造の表示
$ angular-structure-analyzer

# コンポーネントの詳細表示
$ angular-structure-analyzer components --detailed

# JSON形式での出力
$ angular-structure-analyzer --json

# 特定のディレクトリの解析
$ angular-structure-analyzer /path/to/angular/project

# 依存関係グラフの表示
$ angular-structure-analyzer dependencies --graph

# テストファイルを含める
$ angular-structure-analyzer --include-tests
```

このツールを使えば、複雑な Angular プロジェクトの構造を視覚的に把握でき、開発効率の向上やコードレビューの補助に役立ちます。

素晴らしい質問です！Angular構造解析ツールの出力に表示される文字（O、C、I、S など）は、ファイルタイプを示す識別子です。これらはファイルの種類を視覚的に区別するために使用されます。

## ファイルタイプ識別子の意味

```
[C] - Component（コンポーネント）
[S] - Service（サービス）
[M] - Module（モジュール）
[D] - Directive（ディレクティブ）
[P] - Pipe（パイプ）
[G] - Guard（ガード）
[R] - Resolver（リゾルバー）
[I] - Interface/Model（インターフェース/モデル）
[CF] - Config File（設定ファイル）
[ST] - Style（スタイルファイル）
[T] - Test（テストファイル）
[O] - Other（その他のファイル）
```

## 解析の例

以下は、実際の Angular プロジェクトを解析した場合の出力例です：

### 基本的なプロジェクト構造の表示

```bash
cargo run -- /path/to/angular/project
```

出力例：
```
INFO: Analyzing project structure...

STRUCTURE: Project Structure:
my-angular-app
  [CF] angular.json
  [CF] package.json
  [CF] tsconfig.json
  src/
    [O] main.ts
    [O] index.html
    [ST] styles.css
    app/
      [C] app.component.ts
      [C] app.component.html
      [ST] app.component.css
      [T] app.component.spec.ts
      [M] app.module.ts
      components/
        users/
          [C] user-list.component.ts
          [C] user-list.component.html
          [ST] user-list.component.css
          [T] user-list.component.spec.ts
        dashboard/
          [C] dashboard.component.ts
          [C] dashboard.component.html
          [ST] dashboard.component.css
      services/
        [S] auth.service.ts
        [S] user.service.ts
        [T] user.service.spec.ts
      models/
        [I] user.model.ts
```

### コンポーネント分析の例

```bash
cargo run -- /path/to/angular/project components --detailed
```

出力例：
```
INFO: Analyzing components...

COMPONENTS: Components (4):
  AppComponent (/path/to/angular/project/src/app/app.component.ts)
    Selector: app-root
    Template: /path/to/angular/project/src/app/app.component.html
    Styles: /path/to/angular/project/src/app/app.component.css
    Test: /path/to/angular/project/src/app/app.component.spec.ts

  UserListComponent (/path/to/angular/project/src/app/components/users/user-list.component.ts)
    Selector: app-user-list
    Template: /path/to/angular/project/src/app/components/users/user-list.component.html
    Styles: /path/to/angular/project/src/app/components/users/user-list.component.css
    Test: /path/to/angular/project/src/app/components/users/user-list.component.spec.ts

  DashboardComponent (/path/to/angular/project/src/app/components/dashboard/dashboard.component.ts)
    Selector: app-dashboard
    Template: /path/to/angular/project/src/app/components/dashboard/dashboard.component.html
    Styles: /path/to/angular/project/src/app/components/dashboard/dashboard.component.css
```

### サービス分析の例

```bash
cargo run -- /path/to/angular/project services
```

出力例：
```
INFO: Analyzing services...

SERVICES: Services (2):
  AuthService (/path/to/angular/project/src/app/services/auth.service.ts)
  UserService (/path/to/angular/project/src/app/services/user.service.ts)
```

### モジュール分析の例

```bash
cargo run -- /path/to/angular/project modules --detailed
```

出力例：
```
INFO: Analyzing modules...

MODULES: Modules (1):
  AppModule (/path/to/angular/project/src/app/app.module.ts)
    Declarations: AppComponent, UserListComponent, DashboardComponent
    Imports: BrowserModule, FormsModule, HttpClientModule
    Providers: AuthService, UserService
    Bootstrap: AppComponent
```

### 依存関係分析の例

```bash
cargo run -- /path/to/angular/project dependencies
```

出力例：
```
INFO: Analyzing dependencies...

DEPENDENCIES: Dependencies (8):
  /path/to/angular/project/src/app/app.component.ts:
    @angular/core -> Component
    ./services/user.service -> Service

  /path/to/angular/project/src/app/components/users/user-list.component.ts:
    @angular/core -> Component
    ../../services/user.service -> Service
    ../../models/user.model -> Model

  /path/to/angular/project/src/app/services/user.service.ts:
    @angular/core -> Module
    @angular/common/http -> Module
    ../models/user.model -> Model
```

### ルート分析の例

```bash
cargo run -- /path/to/angular/project routes
```

出力例：
```
INFO: Analyzing routes...

ROUTES: Routes (4):
  / -> AppComponent
  users -> UserListComponent
  dashboard -> DashboardComponent
  user/:id (lazy: ./user/user.module#UserModule)
    profile -> UserProfileComponent
    edit -> UserEditComponent
```

これらの例は、ツールがどのようにAngularプロジェクトを解析し、その結果を視覚的に表示するかを示しています。各ファイルタイプ識別子はカラーコード化されており、コンソール出力では異なる色で表示され、プロジェクト構造をより理解しやすくしています。
