# bevy-tutorial

## アプリのスケジュールについて
- 初回に一回だけ
  - StartUp
    - Window作成やMIDIの読み込み、カメラの設定等
  - PostStartUp
    - StartUp時点でMIDIやカメラが読みこまれるので、それを描画する
    - 初期の描画は全てPostStartUpで行う
- 毎フレーム
  - PreUpdate
    - ステータスやTimeAxisの更新
  - Update
    - PreUpdateによって更新された情報の描画