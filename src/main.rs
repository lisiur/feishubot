#![allow(dead_code)]

use std::env;

fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let message = Message::new_post(
        "测试富文本",
        vec![vec![
            PostTag::Text {
                text: "text".to_string(),
                un_escape: None,
            },
            PostTag::A {
                text: "link".to_string(),
                un_escape: None,
                href: "https://lisiur.com".to_string(),
            },
        ]],
    );

    message.send()?;

    Ok(())
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "tag")]
enum PostTag {
    Text {
        /// 显示的默认的文本内容，如果设置了 i18n 内容，会优先显示 i18n 里面对应的语种内容
        ///
        /// # 示例
        /// test text
        text: String,

        /// 表示是不是 unescape 解码，默认为 false ，不用可以不填
        #[serde(skip_serializing_if = "Option::is_none")]
        un_escape: Option<bool>,
    },
    A {
        /// 显示的默认的文本内容，如果设置了 i18n 内容，会优先显示 i18n 里面对应的语种内容
        ///
        /// # 示例
        /// test text
        text: String,

        /// 表示是不是 unescape 解码，默认为 false ，不用可以不填
        #[serde(skip_serializing_if = "Option::is_none")]
        un_escape: Option<bool>,

        /// 默认的链接地址
        ///
        /// # 示例
        /// https://bytedance.com
        href: String,
    },
    At {
        /// open_id 或者 user_id
        ///
        /// # 示例
        /// ou_18eac85d35a26f989317ad4f02e8bbbb
        user_id: String,
    },
    Img {
        /// 图片的唯一标识，可以通过[图片上传接口](https://open.feishu.cn/document/ukTMukTMukTM/uEDO04SM4QjLxgDN)获得
        ///
        /// # 示例
        /// d640eeea-4d2f-4cb3-88d8-c964fab53987
        image_key: String,

        /// 图片的高
        height: u32,

        /// 图片的宽
        width: u32,
    },
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum PostBody {
    ZhCn(PostBodyContent),
    JaJp(PostBodyContent),
    EnUs(PostBodyContent),
}

#[derive(serde::Serialize)]
struct PostBodyContent {
    title: String,
    content: Vec<Vec<PostTag>>,
}

#[derive(serde::Serialize)]
struct InteractiveConfig {
    /// 是否允许卡片被转发。 默认 true，客户端版本要求为3.31.0
    ///
    /// # 注意
    /// 1. 2020/09/24之后，缺省值将变更为true，请有需要禁止转发的卡片务必声明此字段
    /// 2. 转发后，卡片上的“回传交互”组件将自动置为禁用态。用户不能在转发后的卡片操作提交数据
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_forward: Option<bool>,

    /// 是否为共享卡片。
    ///
    /// `true`：是共享卡片，更新卡片的内容对所有收到这张卡片的人员可见。  
    ///
    /// `false`：是独享卡片，即仅操作用户可见卡片的更新内容。  
    ///
    /// 默认为 false  
    #[serde(skip_serializing_if = "Option::is_none")]
    update_multi: Option<bool>,
}

#[derive(serde::Serialize)]
struct InteractiveHeader {
    /// 配置卡片标题内容
    title: String,

    /// 控制标题背景颜色
    ///
    /// 取值范围： `blue`、`wathet`、`turquoise`、`green`、`yellow`、`orange`、`red`、`carmine`、`violet`、`purple`、`indigo`、`grey`
    template: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "tag")]
enum InteractiveElement {
    Div {
        /// 单个文本展示，和 `fields` 至少要有一个
        text: Option<Text>,

        /// 多个文本展示，和 `text` 至少要有一个
        fields: Option<Vec<Field>>,

        /// 附加的元素展示在文本内容右侧。
        extra: Option<()>,
    },
    Hr,
    Img {
        /// 图片资源
        img_key: String,

        /// hover图片时弹出的Tips文案, content取值为空时则不展示
        alt: Text,

        /// 图片标题
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<Text>,

        /// 自定义图片的最大展示宽度。 默认展示宽度撑满卡片的通栏图片，可在 278px~580px 范围内指定最大展示宽度。
        custom_width: Option<u32>,

        /// 是否展示为紧凑型的图片。默认为false，若配置为true，则展示最大宽度为 278px 的紧凑型图片。
        compact_width: Option<bool>,

        /// 图片显示模式。
        ///
        /// - `crop_center`： 居中裁剪模式，对长图会限高，并居中裁剪后展示
        /// - `fit_horizontal`： 平铺模式，宽度撑满卡片完整展示上传的图片。该属性会覆盖 `custom_width` 属性。
        mode: Option<String>,

        /// 点击后是否放大图片，缺省为true。
        preview: Option<bool>,
    },
    Action {
        /// 放置交互元素
        ///
        /// # 注意
        /// 只能使用 Extra::Button
        actions: Vec<Extra>,

        /// 交互元素布局，窄版样式默认纵向排列
        ///
        /// - 使用 `bisected` 为二等分布局，每行两列交互元素
        /// - 使用 `trisection` 为三等分布局，每行三列交互元素
        /// - 使用 `flow` 为流式布局元素会按自身大小横向排列并在空间不够的时候折行
        #[serde(skip_serializing_if = "Option::is_none")]
        layout: Option<String>,
    },
    Note {
        elements: Vec<Note>,
    },
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "tag")]
enum Note {
    Text(Text),
    Img(Img),
}

#[derive(serde::Serialize)]
struct Img {
    img_key: String,
    alt: Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<bool>,
}

#[derive(serde::Serialize)]
struct Field {
    /// 是否并排布局
    is_short: bool,

    /// 国际化文本内容
    text: Text,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "tag")]
enum Extra {
    Button {
        text: Text,

        /// 跳转链接，和multi_url互斥
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,

        /// 多端跳转链接
        #[serde(skip_serializing_if = "Option::is_none")]
        multi_url: Option<MultiUrl>,

        /// 配置按钮样式，默认为 "default"
        ///
        /// 可选值： `default`、`primary`、`danger`
        #[serde(skip_serializing_if = "Option::is_none")]
        r#type: Option<String>,

        /// 点击后返回业务方
        ///
        /// # 注意
        /// 仅支持 key-value 形式的 json 结构，且 key 为 String 类型。
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,

        /// 二次确认的弹框
        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<Confirm>,
    },
    Img(Img),
    SelectStatic {},
    SelectPerson {},
    Overflow {
        /// TODO
        options: (),

        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,

        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<Confirm>,
    },
    DatePicker {
        /// 初始值，格式为 yyyy-MM-dd
        #[serde(skip_serializing_if = "Option::is_none")]
        initial_date: Option<String>,

        ///占位符，无初始值时必填
        #[serde(skip_serializing_if = "Option::is_none")]
        placeholder: Option<Text>,

        /// 用户选定后返回业务方的数据
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,

        /// 二次确认的弹框
        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<Confirm>,
    },
    PickerTime {
        /// 初始值，格式为 HH:mm
        #[serde(skip_serializing_if = "Option::is_none")]
        initial_time: Option<String>,

        ///占位符，无初始值时必填
        #[serde(skip_serializing_if = "Option::is_none")]
        placeholder: Option<Text>,

        /// 用户选定后返回业务方的数据
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,

        /// 二次确认的弹框
        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<Confirm>,
    },
    PickerDateTime {
        /// 初始值，格式为 yyyy-MM-dd HH:mm
        #[serde(skip_serializing_if = "Option::is_none")]
        initial_datetime: Option<String>,

        ///占位符，无初始值时必填
        #[serde(skip_serializing_if = "Option::is_none")]
        placeholder: Option<Text>,

        /// 用户选定后返回业务方的数据
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,

        /// 二次确认的弹框
        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<Confirm>,
    },
}

#[derive(serde::Serialize)]
struct Confirm {
    /// 弹框标题
    title: Text,

    /// 弹框内容
    text: Text,
}

#[derive(serde::Serialize)]
struct MultiUrl {
    /// 默认跳转链接
    url: String,

    /// 安卓端跳转链接
    android_url: String,

    /// ios 端跳转链接
    ios_url: String,

    /// pc 端跳转链接
    pc_url: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "tag")]
enum Text {
    PlainText {
        content: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        lines: Option<u32>,
    },
    LarkMd {
        content: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        lines: Option<u32>,
    },
}

#[derive(serde::Serialize)]
struct InteractiveBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<InteractiveConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<InteractiveHeader>,

    elements: Vec<InteractiveElement>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "msg_type", content = "content")]
enum Message {
    Text { text: String },
    Post { post: PostBody },
    Interactive { card: InteractiveBody },
}

impl Message {
    pub fn new_text(text: &str) -> Self {
        Message::Text {
            text: text.to_string(),
        }
    }
    pub fn new_post(title: &str, content: Vec<Vec<PostTag>>) -> Self {
        Message::Post {
            post: PostBody::ZhCn(PostBodyContent {
                title: title.to_string(),
                content,
            }),
        }
    }
    pub fn new_card(card: InteractiveBody) -> Self {
        Message::Interactive { card }
    }

    pub fn send(&self) -> anyhow::Result<()> {
        let token = std::env::var("FEISHU_TOKEN")
            .unwrap_or_else(|_| "7c161133-e77c-4df8-bd5c-8ae1c1700bf6".to_string());
        let url = format!("https://open.feishu.cn/open-apis/bot/v2/hook/{}", token);

        let json_data = serde_json::to_string(self)?;
        log::info!("send reqwest: {}", json_data);
        let res = reqwest::blocking::Client::new()
            .post(&url)
            .body(json_data)
            .send()?;

        let res_data = res.json::<serde_json::Value>()?;
        log::info!("get response: {}", res_data);

        Ok(())
    }
}
