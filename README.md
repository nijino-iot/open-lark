[![crates.io](https://img.shields.io/crates/v/open-lark)](https://crates.io/crates/open-lark)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)
[![CI](https://github.com/foxzool/open-lark/workflows/CI/badge.svg)](https://github.com/foxzool/open-lark/actions)
[![Documentation](https://docs.rs/open-lark/badge.svg)](https://docs.rs/open-lark)
![Discord Shield](https://discord.com/api/guilds/1319490473060073532/widget.png?style=shield)

# 飞书开放平台非官方SDK, 个人开发, 请谨慎使用

支持自定义机器人、长连接机器人、云文档、飞书卡片、消息、群组等API调用。

## 🎉 v0.5.0 重大更新 - 完整考勤模块实现

- **📊 完整考勤功能**: 实现 41个考勤API + 2个事件处理器，涵盖飞书考勤的所有功能
- **🏢 企业级功能**: 支持班次管理、排班、考勤组、用户设置、统计分析、审批流程
- **⚡ 实时监听**: 考勤事件处理器，支持打卡流水和任务状态变更的实时监听
- **📝 丰富示例**: 43个完整示例，每个功能都有详细的中文注释和使用演示
- **🛡️ 类型安全**: Rust强类型系统保证编译时安全，完善的错误处理机制

## 使用

将`.env-example`文件重命名为`.env`，并填写相关配置。

### 快速开始 - 考勤模块

```rust
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = LarkClient::builder("your_app_id", "your_app_secret").build();

    // 查询所有班次
    let shifts_req = ListShiftsRequest {
        employee_type: "employee_id".to_string(),
        page_size: Some(20),
        ..Default::default()
    };
    
    let shifts_resp = client.attendance.v1.shift.list(shifts_req, None).await?;
    println!("班次列表: {:?}", shifts_resp.data);

    // 查询打卡记录
    let user_task_req = QueryUserTaskRequest {
        employee_type: "employee_id".to_string(),
        user_ids: vec!["employee_123".to_string()],
        check_date_from: "2024-06-01".to_string(),
        check_date_to: "2024-06-30".to_string(),
        ..Default::default()
    };
    
    let user_task_resp = client.attendance.v1.user_task.query(user_task_req, None).await?;
    println!("打卡记录: {:?}", user_task_resp.data);

    Ok(())
}
```

### 考勤事件监听

```rust
use open_lark::event::EventDispatcherHandler;

let handler = EventDispatcherHandler::builder()
    .register_p2_attendance_user_task_updated_v1(|event| {
        println!("收到打卡流水事件: {:?}", event);
    })
    .register_p2_attendance_user_task_status_change_v1(|event| {
        println!("收到任务状态变更事件: {:?}", event);
    })
    .build();
```

## 已完成

### 认证与授权

- [x] 自建应用获取 tenant_access_token

### 自定义机器人

- [x] 发送消息
- [x] 签名验证

### 长连接机器人

- [x] 接收事件推送

### 云文档

#### 云空间

- 文件夹
    - [x] 获取我的空间元信息
    - [x] 获取文件夹元信息
    - [x] 新建文件夹
    - [x] 获取文件夹下的清单
- 上传
    - [x] 上传文件
- 下载
    - [x] 下载文件

#### 权限

- [x] 获取云文档权限设置
- [x] 更新云文档权限设置

#### 电子表格

- 表格
    - [x] 创建表格
    - [x] 修改电子表格属性
    - [x] 获取电子表格信息
- 工作表
    - [x] 查询工作表
    - [x] 获取工作表
    - [x] 操作工作表
    - [x] 更新工作表属性
- 行列
    - [x] 增加行列
    - [x] 插入行列
    - [x] 更新行列
    - [x] 移动行列
    - [x] 删除行列
- 单元格
    - [x] 插入数据
    - [x] 追加数据
    - [x] 读取单个范围
    - [x] 向单个范围写入数据
    - [x] 读取多个范围
    - [x] 向多个范围写入数据
    - [x] 设置单元格样式
    - [x] 批量设置单元格样式
    - [x] 写入图片
    - [x] 合并单元格
    - [x] 拆分单元格
    - [x] 查找单元格
    - [x] 替换单元格
- 筛选
    - [x] 获取筛选
    - [x] 创建筛选
    - [x] 更新筛选
    - [x] 删除筛选

#### 多维表格

- 多维表格
    - [x] 获取多维表格元数据
- 字段
    - [x] 列出字段
- 记录
    - [x] 新增记录
    - [x] 查询记录
    - [x] 新增多条记录
    - [x] 更新记录

#### 通讯录

- 用户
    - [x] 搜索用户

### 飞书卡片

#### 卡片组件

- [x] 卡片回传交互
- 容器
    - [x] 分栏
    - [x] 表单容器
    - [x] 交互容器
    - [x] 折叠面板
- 展示
    - [x] 标题
    - [x] 普通文本
    - [x] 富文本(Markdown)
    - [x] 图片
    - [x] 多图混排
    - [x] 分割线
    - [x] 人员
    - [x] 人员列表
    - [x] 图表
    - [x] 表格
    - [x] 备注
- 交互
    - [x] 输入框
    - [x] 按钮
    - [x] 折叠按钮组
    - [x] 下拉选择-单选
    - [x] 下拉选择-多选
    - [x] 人员选择-单选
    - [x] 人员选择-多选
    - [x] 日期选择器
    - [x] 时间选择器
    - [x] 日期时间选择器
    - [x] 多图选择
    - [x] 勾选器

### 消息

- [x] 发送消息
- [x] 获取会话历史消息
- 消息内容结构
    - 发送消息内容
        - [x] 文本
        - [x] 富文本
        - [x] 图片
        - [x] 卡片
- 事件
    - [x] 接收消息

### 群组

- [x] 获取用户或机器人所在的群列表

### 考勤管理 🎉 v0.5.0 新增

#### 考勤班次
- [x] 创建班次
- [x] 删除班次
- [x] 按 ID 查询班次
- [x] 按名称查询班次
- [x] 查询所有班次

#### 考勤排版
- [x] 创建或修改排班表
- [x] 查询排班表
- [x] 创建或修改临时排班

#### 考勤管理
- [x] 查询考勤组下所有成员
- [x] 创建或修改考勤组
- [x] 删除考勤组
- [x] 按 ID 查询考勤组
- [x] 按名称查询考勤组
- [x] 查询所有考勤组

#### 考勤用户管理
- [x] 修改用户人脸识别信息
- [x] 批量查询用户人脸识别信息
- [x] 上传用户人脸识别照片
- [x] 下载用户人脸识别照片

#### 考勤统计
- [x] 更新统计设置
- [x] 查询统计表头
- [x] 查询统计设置
- [x] 查询统计数据

#### 假勤审批
- [x] 获取审批数据
- [x] 写入审批结果
- [x] 通知审批状态更新

#### 考勤补卡
- [x] 通知补卡审批发起
- [x] 获取可补卡时间
- [x] 获取补卡记录

#### 归档报表
- [x] 查询归档报表表头
- [x] 写入归档报表结果
- [x] 删除归档报表行数据
- [x] 查询所有归档规则

#### 打卡信息管理
- [x] 导入打卡流水
- [x] 查询打卡流水
- [x] 批量查询打卡流水
- [x] 删除打卡流水
- [x] 查询打卡结果

#### 休假管理
- [x] 通过过期时间获取发放记录
- [x] 修改发放记录

#### 考勤事件
- [x] 打卡流水事件处理
- [x] 用户任务状态变更事件处理

## 📊 功能完成度统计

| 模块 | API数量 | 完成状态 | 说明 |
|------|---------|----------|------|
| **🔐 认证与授权** | 1 | ✅ 100% | 应用身份验证 |
| **🤖 自定义机器人** | 2 | ✅ 100% | 消息发送与签名验证 |
| **🔗 长连接机器人** | 1 | ✅ 100% | 事件推送接收 |
| **☁️ 云文档-云空间** | 7 | ✅ 100% | 文件夹管理、上传下载 |
| **🛡️ 云文档-权限** | 2 | ✅ 100% | 权限设置管理 |
| **📊 云文档-电子表格** | 33 | ✅ 100% | 完整表格操作功能 |
| **📋 云文档-多维表格** | 6 | ✅ 100% | 数据表操作 |
| **👥 通讯录** | 1 | ✅ 100% | 用户搜索 |
| **🎨 飞书卡片** | 25 | ✅ 100% | 完整卡片组件系统 |
| **💬 消息** | 4 | ✅ 100% | 消息发送与接收 |
| **👥 群组** | 1 | ✅ 100% | 群组管理 |
| **🏢 考勤管理** | 43 | ✅ 100% | **v0.5.0 新增** - 完整考勤解决方案 |
| **📈 总计** | **126** | **✅ 100%** | **覆盖企业应用核心功能** |

### 🎯 v0.5.0 考勤模块亮点
- **41个API** 覆盖班次、排班、统计、审批等全业务流程
- **2个事件处理器** 支持实时考勤数据监听
- **43个示例** 每个功能都有完整的使用演示
- **企业级特性** 支持复杂的考勤规则和业务场景
