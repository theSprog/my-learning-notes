### Settings Sync

`Settings Sync` 是用于在多端主机上同步 VSCode 配置的一款插件



### 步骤

- 登录 github

- 点击头像 -> settings -> Developer settings

- 点击 personal access tokens

  <img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220921112308587.png" alt="image-20220921112308587" style="zoom: 67%;" />

- 如有必要，清除已过期的 tokens
- 点击 `Generate new token`
- note 填写名称（任意名称均可），推荐 `vscode settings sync`
- 选定过期时间，勾选 `gist`，最后点击 `Generate token`，拷贝好生成的 `tokens`
- 注意保管好这个 tokens，之后同步配置要用



- 回到 VSCode，点击 `Edit Configuration`

  <img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220921112923229.png" alt="image-20220921112923229" style="zoom:50%;" />

- 在获取令牌那一栏填入之前复制的 tokens

  <img src="https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220921113042037.png" alt="image-20220921113042037" style="zoom:50%;" />

- `Shift + Alt + U` 是上传，`Shift + Alt + D` 是下载

- 回到 github，点击头像 -> `Your gists`，就可以看到刚才上传的配置

  

  

### 配置同步

在一台新的主机上，我们安装好 settings sync 后

- 点击 `Edit Configuration`，需要填写两个部分，一个是之前生成的 token，另一个是 gist ID

- gist ID 可以到 `Your gists` 中去 Embed 获取，这个 URL 中就包含 gist ID

  ![image-20220921114638366](https://src-1259777572.cos.ap-chengdu.myqcloud.com/image-20220921114638366.png)

- 将两部分填好后，回到一个正常页面，按下快捷键 `Shift + Alt + D`，即可开始同步配置