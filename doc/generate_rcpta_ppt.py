#!/usr/bin/env python3
from __future__ import annotations

import html
import shutil
import zipfile
from pathlib import Path


OUT = Path("doc/rcpta_project_report_template_style.pptx")
IMG = Path("doc/rcpta_arc_v3_purple.png")
SEG_LOGO = Path("/tmp/rcpta_template_media/ppt/media/image1.png")
NJU_LOGO = Path("/tmp/rcpta_template_media/ppt/media/image2.png")
TMP = Path("/tmp/rcpta_pptx_build")

W = 12192000
H = 6858000

FONT = "Arial"
TITLE_FONT = "Arial Black"
BLUE = "2F5597"
DARK = "111111"
TEXT = "222222"
MUTED = "666666"
LIGHT = "FFFFFF"
LINE = "D9DCE2"
GREEN = "548235"
RED = "A61C1C"
YELLOW = "C19859"
ORANGE = "604878"
PANEL = "F4F5F7"
SOFT_BLUE = "EAF1F8"


def esc(s: str) -> str:
    return html.escape(s, quote=True)


def emu(inch: float) -> int:
    return int(inch * 914400)


def rgb(fill: str | None) -> str:
    if fill is None:
        return "<a:noFill/>"
    return f'<a:solidFill><a:srgbClr val="{fill}"/></a:solidFill>'


def ln(color: str = LINE, width: int = 12700) -> str:
    return f'<a:ln w="{width}"><a:solidFill><a:srgbClr val="{color}"/></a:solidFill></a:ln>'


def run(text: str, size: int = 18, color: str = TEXT, bold: bool = False, font: str | None = None) -> str:
    b = ' b="1"' if bold else ""
    typeface = font or FONT
    return (
        f'<a:r><a:rPr lang="zh-CN" sz="{size * 100}"{b}>'
        f'<a:latin typeface="{typeface}"/><a:ea typeface="{typeface}"/><a:cs typeface="{typeface}"/>'
        f'<a:solidFill><a:srgbClr val="{color}"/></a:solidFill>'
        f'</a:rPr><a:t>{esc(text)}</a:t></a:r>'
    )


def para(text: str, size: int = 18, color: str = TEXT, bold: bool = False,
         align: str = "l", bullet: bool = False, font: str | None = None) -> str:
    ppr = f'<a:pPr algn="{align}">'
    if bullet:
        ppr += '<a:buChar char="•"/>'
    ppr += '</a:pPr>'
    return f"<a:p>{ppr}{run(text, size, color, bold, font)}</a:p>"


def textbox(idx: int, x: int, y: int, w: int, h: int, paragraphs: list[str],
            fill: str | None = None, line: str | None = None, radius: str = "roundRect",
            anchor: str = "top") -> str:
    line_xml = '<a:ln><a:noFill/></a:ln>' if line is None else ln(line)
    return f"""
<p:sp>
  <p:nvSpPr><p:cNvPr id="{idx}" name="TextBox {idx}"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
  <p:spPr>
    <a:xfrm><a:off x="{x}" y="{y}"/><a:ext cx="{w}" cy="{h}"/></a:xfrm>
    <a:prstGeom prst="{radius}"><a:avLst/></a:prstGeom>
    {rgb(fill)}
    {line_xml}
  </p:spPr>
  <p:txBody>
    <a:bodyPr wrap="square" anchor="{anchor}" lIns="91440" rIns="91440" tIns="60960" bIns="60960"/>
    <a:lstStyle/>
    {''.join(paragraphs)}
  </p:txBody>
</p:sp>
"""


def rect(idx: int, x: int, y: int, w: int, h: int, fill: str, outline: str = LINE) -> str:
    return f"""
<p:sp>
  <p:nvSpPr><p:cNvPr id="{idx}" name="Rect {idx}"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
  <p:spPr>
    <a:xfrm><a:off x="{x}" y="{y}"/><a:ext cx="{w}" cy="{h}"/></a:xfrm>
    <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
    {rgb(fill)}
    {ln(outline)}
  </p:spPr>
</p:sp>
"""


def image(idx: int, rel_id: str, x: int, y: int, w: int, h: int) -> str:
    return f"""
<p:pic>
  <p:nvPicPr><p:cNvPr id="{idx}" name="Picture {idx}"/><p:cNvPicPr/><p:nvPr/></p:nvPicPr>
  <p:blipFill><a:blip r:embed="{rel_id}"/><a:stretch><a:fillRect/></a:stretch></p:blipFill>
  <p:spPr><a:xfrm><a:off x="{x}" y="{y}"/><a:ext cx="{w}" cy="{h}"/></a:xfrm><a:prstGeom prst="rect"><a:avLst/></a:prstGeom></p:spPr>
</p:pic>
"""


def title_shape(idx: int, title: str, subtitle: str | None = None) -> str:
    shapes = [
        rect(idx + 500, emu(0.0), emu(0.0), emu(0.18), emu(7.5), ORANGE, ORANGE),
        rect(idx + 501, emu(0.45), emu(0.95), emu(11.95), 1, LINE, LINE),
    ]
    ps = [para(title, 32, DARK, True, "l", False, TITLE_FONT)]
    if subtitle:
        ps.append(para(subtitle, 16, MUTED, False, "l"))
    shapes.append(textbox(idx, emu(0.55), emu(0.23), emu(11.5), emu(0.78), ps, None, None, "rect"))
    return "".join(shapes)


def footer(idx: int) -> str:
    return (
        rect(idx + 600, emu(0.55), emu(6.94), emu(11.9), 1, LINE, LINE)
        + image(idx + 650, "rId3", emu(0.55), emu(7.0), emu(0.43), emu(0.27))
        + textbox(idx, emu(0.55), emu(7.02), emu(11.8), emu(0.24),
                  [para("RCPTA 项目汇报 | Rust OO-style DSL Static Analysis", 9, MUTED, False, "r")],
                  None, None)
    )


def kpi(idx: int, x: float, y: float, w: float, num: str, label: str, color: str = BLUE) -> str:
    return textbox(idx, emu(x), emu(y), emu(w), emu(0.9),
                   [para(num, 35, color, True, "ctr", False, TITLE_FONT), para(label, 14, MUTED, False, "ctr")],
                   None, None, "rect", "mid")


def bullet_box(idx: int, x: float, y: float, w: float, h: float, title: str, bullets: list[str],
               accent: str = BLUE) -> str:
    label_w = min(1.65, max(1.15, len(title) * 0.12))
    body_size = 15 if len(bullets) <= 3 else 14
    return (
        textbox(idx + 700, emu(x), emu(y), emu(label_w), emu(0.34),
                [para(title, 13, "FFFFFF", True, "ctr")], accent, accent, "rect", "mid")
        + textbox(idx, emu(x + 0.02), emu(y + 0.5), emu(w - 0.04), emu(h - 0.5),
                  [para(b, body_size, TEXT, False, "l", True) for b in bullets],
                  None, None, "rect")
    )


def stage_box(idx: int, x: float, y: float, w: float, h: float, title: str, lines: list[str], color: str) -> str:
    ps = [para(title, 18, color, True, "ctr")] + [para(s, 13, MUTED, False, "ctr") for s in lines]
    return textbox(idx, emu(x), emu(y), emu(w), emu(h), ps, SOFT_BLUE if color == BLUE else "FFFFFF", color, "rect", "mid")


def arrow(idx: int, x1: float, y1: float, x2: float, y2: float, color: str = BLUE) -> str:
    return f"""
<p:cxnSp>
  <p:nvCxnSpPr><p:cNvPr id="{idx}" name="Arrow {idx}"/><p:cNvCxnSpPr/><p:nvPr/></p:nvCxnSpPr>
  <p:spPr>
    <a:xfrm><a:off x="{emu(min(x1, x2))}" y="{emu(min(y1, y2))}"/><a:ext cx="{abs(emu(x2 - x1))}" cy="{abs(emu(y2 - y1))}"/></a:xfrm>
    <a:prstGeom prst="straightConnector1"><a:avLst/></a:prstGeom>
    <a:ln w="28575"><a:solidFill><a:srgbClr val="{color}"/></a:solidFill><a:tailEnd type="none"/><a:headEnd type="triangle"/></a:ln>
  </p:spPr>
</p:cxnSp>
"""


def slide_xml(shapes: list[str]) -> str:
    return f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:bg><p:bgPr>{rgb(LIGHT)}<a:effectLst/></p:bgPr></p:bg>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="{W}" cy="{H}"/><a:chOff x="0" y="0"/><a:chExt cx="{W}" cy="{H}"/></a:xfrm></p:grpSpPr>
      {''.join(shapes)}
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>
</p:sld>
"""


def rels_xml(image_rel: bool = False) -> str:
    image_entry = (
        '<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/rcpta_arc_v3.png"/>'
        '<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/template_seg.png"/>'
        '<Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/template_nju.png"/>'
    )
    if not image_rel:
        image_entry = (
            '<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/template_seg.png"/>'
            '<Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="../media/template_nju.png"/>'
        )
    return f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
  {image_entry}
</Relationships>
"""


def make_slides() -> list[tuple[str, bool]]:
    slides: list[tuple[str, bool]] = []

    shapes = [
        rect(2, emu(0), emu(0), emu(0.26), emu(7.5), ORANGE, ORANGE),
        image(3, "rId4", emu(0.72), emu(0.48), emu(1.9), emu(0.61)),
        image(4, "rId3", emu(11.85), emu(0.48), emu(0.72), emu(0.46)),
        textbox(5, emu(0.82), emu(1.35), emu(11.2), emu(1.55),
                [para("RCPTA 两阶段静态分析", 42, DARK, True, "l", False, TITLE_FONT),
                 para("与 Cast Site 诊断工具", 39, DARK, True, "l", False, TITLE_FONT)],
                None, None, "rect", "mid"),
        textbox(6, emu(0.88), emu(3.05), emu(10.9), emu(0.72),
                [para("面向 Rust OO-style DSL / Lite Class DSL 的类级 points-to 分析与 checked cast 风险诊断", 20, MUTED, False, "l")],
                None, None, "rect", "mid"),
        rect(7, emu(0.88), emu(3.82), emu(11.2), 1, LINE, LINE),
        kpi(8, 0.9, 4.2, 2.1, "51", "旧 DSL 全入口可分析", BLUE),
        kpi(9, 3.4, 4.2, 2.1, "69", "Lite 原始套件结果", GREEN),
        kpi(10, 5.9, 4.2, 2.1, "90", "Cast Risk Matrix", ORANGE),
        kpi(11, 8.4, 4.2, 2.1, "0", "最终 unknown", RED),
        textbox(12, emu(0.9), emu(5.68), emu(11.0), emu(0.88),
                [para("核心贡献：从 MIR 层复杂展开中恢复 class/interface/mixin 对象流，输出可审计的动态类型集合与 cast 安全分类。", 18, TEXT, False, "l")],
                None, None, "rect", "mid"),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "1. 项目问题与目标", "Rust 宏与泛型展开后，源码级 OO 语义在 MIR 中被打散")]
    shapes += [
        bullet_box(3, 0.75, 1.35, 3.7, 4.75, "分析难点", [
            "类对象流跨 clone、move、引用临时变量传播",
            "Option / Result / 容器 API 会打断 payload 路径",
            "interface / mixin / downcast 需要源码级类型关系",
            "普通 MIR points-to 难以直接解释源码 cast site",
        ], BLUE),
        bullet_box(4, 4.8, 1.35, 3.7, 4.75, "RCPTA 目标", [
            "构建 ClassPtr / ClassObj / ClassPAG 抽象",
            "计算 ClassPTS 和动态类型集合",
            "恢复 class / interface / mixin 继承关系",
            "输出 safe / may-unsafe / must-unsafe / unknown",
        ], GREEN),
        bullet_box(5, 8.85, 1.35, 3.7, 4.75, "最终用途", [
            "解释对象可能流向哪里",
            "解释 receiver 可能有哪些动态类型",
            "为 cast erase 提供静态优化证据",
            "把 unknown 暴露为建模缺口",
        ], ORANGE),
        footer(6),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "2. 两阶段技术路线", "第一阶段建立类级分析基线，第二阶段补齐真实 DSL 与 cast 诊断能力")]
    shapes += [
        stage_box(3, 0.85, 1.55, 3.2, 1.25, "Stage 1: 旧 DSL 基线", [
            "ClassPAG / ClassPTS / ClassCG",
            "animal / shape / vehicle / full 套件",
        ], BLUE),
        arrow(4, 4.05, 2.17, 4.8, 2.17),
        stage_box(5, 4.85, 1.55, 3.2, 1.25, "Stage 2A: 旧 DSL 增强", [
            "proptest 入口下钻",
            "wrapper / container / cast risk detection",
        ], GREEN),
        arrow(6, 8.05, 2.17, 8.8, 2.17),
        stage_box(7, 8.85, 1.55, 3.2, 1.25, "Stage 2B: Lite DSL", [
            "适配 Lite Class DSL",
            "构造 cast_risk_matrix",
        ], ORANGE),
        bullet_box(8, 0.95, 3.35, 3.25, 2.2, "阶段一产出", [
            "44 个入口成功产出",
            "定位 vehicle prop 栈溢出和外部依赖噪声",
            "验证类级对象流抽象可行",
        ], BLUE),
        bullet_box(9, 5.0, 3.35, 3.25, 2.2, "阶段二前半", [
            "旧 DSL 51 个入口全部成功",
            "非空 points-to 比例约 97.35%",
            "253 个旧 DSL cast site 完成诊断",
        ], GREEN),
        bullet_box(10, 9.05, 3.35, 3.25, 2.2, "阶段二后半", [
            "Lite 原始套件 69 个入口",
            "Cast Risk Matrix 90 个入口",
            "最终 0 unknown / 0 空源诊断",
        ], ORANGE),
        footer(11),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "3. 当前 RCPTA 架构", "简化后的两阶段架构图，突出分析主链路和第二阶段语义增强"),
              image(3, "rId2", emu(0.45), emu(1.05), emu(12.45), emu(6.0))]
    slides.append((slide_xml(shapes), True))

    shapes = [title_shape(2, "4. 核心抽象：ClassPAG 与 ClassPTS", "把 MIR 中与 DSL 类引用相关的行为提升为源码级对象流")]
    shapes += [
        bullet_box(3, 0.8, 1.25, 3.6, 4.8, "ClassPtr", [
            "局部变量、参数、返回值",
            "字段、wrapper payload",
            "container element slot / map value slot",
            "cast 源、cast 结果和 canonical path",
        ], BLUE),
        bullet_box(4, 4.8, 1.25, 3.6, 4.8, "ClassObj", [
            "基于分配点的抽象对象",
            "记录动态 class type",
            "对象身份与类型集合解耦",
            "服务动态类型范围计算",
        ], GREEN),
        bullet_box(5, 8.8, 1.25, 3.6, 4.8, "ClassPAG 边", [
            "Alloc / Assign / Cast",
            "Load / Store",
            "CallArg / CallRet",
            "Cast 边保留 pre-cast PTS 快照",
        ], ORANGE),
        footer(6),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "5. Cast Safety 分类逻辑", "用源动态类型集合与目标静态类型关系进行可解释诊断")]
    shapes += [
        stage_box(3, 0.95, 1.35, 2.55, 1.1, "Source", ["pre-cast source PTS", "dynamic type set"], BLUE),
        arrow(4, 3.5, 1.9, 4.35, 1.9),
        stage_box(5, 4.4, 1.35, 2.55, 1.1, "Target", ["static destination type", "class / interface / mixin"], BLUE),
        arrow(6, 6.95, 1.9, 7.8, 1.9),
        stage_box(7, 7.85, 1.35, 2.55, 1.1, "Relation", ["extends* / implements*", "with* closure"], BLUE),
        arrow(8, 10.4, 1.9, 11.1, 1.9),
        stage_box(9, 11.15, 1.35, 1.35, 1.1, "Result", ["classification"], BLUE),
        bullet_box(10, 0.95, 3.05, 2.65, 2.15, "safe", [
            "所有源动态类型都满足目标类型",
            "可作为 cast erase 优化候选",
        ], GREEN),
        bullet_box(11, 3.95, 3.05, 2.65, 2.15, "may-unsafe", [
            "部分满足、部分不满足",
            "当前需保留运行时检查",
        ], ORANGE),
        bullet_box(12, 6.95, 3.05, 2.65, 2.15, "must-unsafe", [
            "所有源动态类型都不满足",
            "表示必然失败路径",
        ], RED),
        bullet_box(13, 9.95, 3.05, 2.65, 2.15, "unknown", [
            "源 PTS 或目标类型证据不足",
            "表示建模缺口",
        ], YELLOW),
        footer(14),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "6. 第二阶段关键语义增强", "高价值修改集中在 wrapper、container、入口恢复和 cast-aware 传播")]
    shapes += [
        bullet_box(3, 0.75, 1.15, 3.7, 2.35, "Wrapper 摘要", [
            "Option / Result unwrap",
            "ok_or / map / and_then",
            "or / or_else / unwrap_or_else fallback",
            "as_ref 引用 wrapper payload bridge",
        ], BLUE),
        bullet_box(4, 4.8, 1.15, 3.7, 2.35, "Container / Iterator", [
            "Vec element summary",
            "iter / into_iter / next / find",
            "collect result element slot",
            "HashMap / BTreeMap value slot",
        ], GREEN),
        bullet_box(5, 8.85, 1.15, 3.7, 2.35, "入口与外部依赖", [
            "proptest wrapper drill-down",
            "业务 closure 补种为分析根",
            "过滤无关外部 framework callee",
            "降低调用图和初始化噪声",
        ], ORANGE),
        bullet_box(6, 0.75, 3.9, 5.75, 1.85, "Cast-aware propagation", [
            "记录 cast 前源 points-to，而不是依赖 cast 后结果",
            "目标类型过滤传播，避免 cast 后自污染",
        ], RED),
        bullet_box(7, 6.85, 3.9, 5.7, 1.85, "当前边界", [
            "主验证路径仍是上下文不敏感、流不敏感",
            "字段覆盖和路径合流会保守地产生 may-unsafe",
        ], YELLOW),
        footer(8),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "7. 旧 DSL 测试结果", "从第一阶段部分入口失败，到第二阶段四套件全入口成功")]
    shapes += [
        kpi(3, 0.85, 1.35, 2.2, "44 / 51", "第一阶段成功入口", BLUE),
        kpi(4, 3.45, 1.35, 2.2, "51 / 51", "第二阶段成功入口", GREEN),
        kpi(5, 6.05, 1.35, 2.2, "917 / 942", "非空 points-to 指针", GREEN),
        kpi(6, 8.65, 1.35, 2.2, "253", "旧 DSL cast sites", BLUE),
        kpi(7, 11.0, 1.35, 1.45, "0", "边界不确定", GREEN),
        bullet_box(8, 0.9, 3.0, 3.6, 2.45, "覆盖套件", [
            "animal_hierarchy: 22 entries",
            "shape_hierarchy: 12 entries",
            "vehicle_hierarchy: 11 entries",
            "rcpta_full_hierarchy: 6 entries",
        ], BLUE),
        bullet_box(9, 4.85, 3.0, 3.6, 2.45, "主要修复效果", [
            "vehicle prop 测试入口不再栈溢出",
            "wrapper/container 传播减少空 points-to",
            "接口多态 receiver 类型可恢复",
        ], GREEN),
        bullet_box(10, 8.8, 3.0, 3.6, 2.45, "Cast 结果", [
            "217 safe",
            "36 unsafe",
            "18 must-unsafe",
            "0 boundary unknown",
        ], ORANGE),
        footer(11),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "8. 新 Lite Class DSL 三套原始测试套件", "这些结果来自 lite_class_dsl/analysis_results/rcpta，而不是旧 rustdsl/classes 套件")]
    shapes += [
        kpi(3, 0.75, 1.25, 2.15, "30", "Lite animal_hierarchy", BLUE),
        kpi(4, 3.1, 1.25, 2.15, "20", "Lite shape_hierarchy", BLUE),
        kpi(5, 5.45, 1.25, 2.15, "19", "Lite vehicle_hierarchy", BLUE),
        kpi(6, 7.8, 1.25, 2.15, "69", "新 DSL 合计入口", GREEN),
        kpi(7, 10.15, 1.25, 2.15, "0", "unknown", GREEN),
        bullet_box(8, 0.75, 2.75, 3.7, 3.05, "新 DSL 聚合规模", [
            "ClassPtr: 6928",
            "ClassObj / Alloc: 789",
            "Assign edges: 1173",
            "Cast edges: 979",
            "Typed ptrs: 2136",
        ], BLUE),
        bullet_box(9, 4.8, 2.75, 3.7, 3.05, "新 DSL Cast 日志", [
            "Safe cast records: 65",
            "Must-unsafe records: 31",
            "No-cast entries: 44",
            "Unknown: 0",
            "未出现 empty points-to 源",
        ], GREEN),
        bullet_box(10, 8.85, 2.75, 3.7, 3.05, "与旧 DSL 区分", [
            "目录：lite_class_dsl/analysis_results/rcpta",
            "主要服务 Lite DSL 本体回归",
            "animal/shape 覆盖 downcast 成功与失败",
            "vehicle 侧重 interface receiver 类型恢复",
            "证明 RCPTA 可运行在新 DSL 真实测试上",
        ], ORANGE),
        footer(11),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "9. Cast Risk Matrix：高价值 Cast Site 数据集", "专门服务 cast erase / cast risk detection 的最终验证矩阵")]
    shapes += [
        kpi(3, 0.85, 1.25, 2.0, "90", "public entries", BLUE),
        kpi(4, 3.2, 1.25, 2.0, "43", "safe", GREEN),
        kpi(5, 5.55, 1.25, 2.0, "40", "may-unsafe", ORANGE),
        kpi(6, 7.9, 1.25, 2.0, "7", "must-unsafe", RED),
        kpi(7, 10.25, 1.25, 2.0, "0", "unknown", GREEN),
        bullet_box(8, 0.85, 2.85, 5.55, 2.75, "覆盖场景", [
            "local object / helper return / multi-level downcast",
            "field store-load / two-holder precision / clone alias chain",
            "Option / Result: unwrap, map, and_then, fallback, as_ref",
            "Vec / iterator / collect, HashMap / BTreeMap value",
        ], BLUE),
        bullet_box(9, 6.85, 2.85, 5.55, 2.75, "收尾质量", [
            "入口数与结果目录数均为 90",
            "未检索到 boundary-unknown",
            "未检索到 empty points-to cast source",
            "interface / mixin 通过 wrapper/container 后仍可诊断",
        ], GREEN),
        footer(10),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "10. 典型修复案例：as_ref 引用 Wrapper", "最后一轮 Lite DSL 扩充暴露并修复的代表性断流问题")]
    shapes += [
        bullet_box(3, 0.85, 1.25, 3.6, 4.35, "问题形态", [
            "Option<CRc<T>>::as_ref()",
            "Result<CRc<T>, E>::as_ref()",
            "返回 Option<&CRc<T>> / Result<&CRc<T>, &E>",
            "旧模型未连接 destination 引用 payload",
        ], RED),
        bullet_box(4, 4.85, 1.25, 3.6, 4.35, "修复策略", [
            "把 receiver payload 接到 destination payload",
            "保留 receiver holder 已有对象流",
            "支持 as_ref().unwrap().clone() 后进入 cast",
            "避免 cast 源 points-to 为空",
        ], BLUE),
        bullet_box(5, 8.85, 1.25, 3.6, 4.35, "验证结果", [
            "Option / Result safe 场景可证明",
            "interface wrapper may-unsafe 可解释",
            "mixin wrapper must-unsafe 可解释",
            "最终矩阵无 unknown",
        ], GREEN),
        footer(6),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "11. 当前成果与工程产物", "RCPTA 已形成可审计、可回归的静态分析与诊断工具链")]
    shapes += [
        bullet_box(3, 0.8, 1.25, 3.65, 4.45, "分析能力", [
            "类级对象流恢复",
            "字段流、函数参数/返回值传播",
            "wrapper/container 摘要",
            "动态类型集合推断",
            "class/interface/mixin 类型关系解析",
        ], BLUE),
        bullet_box(4, 4.8, 1.25, 3.65, 4.45, "输出产物", [
            "class_pag.txt",
            "class_pts.txt / class_cg.txt",
            "type-info.txt",
            "inheritance_graph.txt",
            "cast_safety.log",
            "per-entry analysis_results",
        ], GREEN),
        bullet_box(5, 8.8, 1.25, 3.65, 4.45, "汇报结论", [
            "旧 DSL 四套件全入口可分析",
            "Lite DSL 原始套件兼容性通过",
            "90 个 cast risk matrix 场景完成分类",
            "最终 unknown 和空源问题均清零",
        ], ORANGE),
        footer(6),
    ]
    slides.append((slide_xml(shapes), False))

    shapes = [title_shape(2, "12. 精度审计与不精确来源", "结论关注当前工具诊断与真实 cast 语义之间的出入，而不是未来工作")]
    shapes += [
        kpi(3, 0.75, 1.25, 2.05, "88/90", "exact 三分类正确", GREEN),
        kpi(4, 3.0, 1.25, 2.05, "97.8%", "总体准确率", GREEN),
        kpi(5, 5.25, 1.25, 2.05, "2", "unsafe 假阳性", RED),
        kpi(6, 7.5, 1.25, 2.05, "0", "must-unsafe 错误", GREEN),
        kpi(7, 9.75, 1.25, 2.05, "0", "unknown", GREEN),
        bullet_box(8, 0.75, 2.75, 3.7, 3.15, "真实语义口径", [
            "90 个 cast matrix 场景按源码意图审计",
            "真实 must-safe: 45",
            "真实 may-unsafe: 38",
            "真实 must-unsafe: 7",
            "工具输出：43 safe / 40 may / 7 must",
        ], BLUE),
        bullet_box(9, 4.8, 2.75, 3.7, 3.15, "假阳性来源", [
            "false positive = 真实安全但被报 may-unsafe",
            "two_holder 精度：两个 holder 的字段内容被合并",
            "field overwrite：流不敏感保留旧写入 Cat",
            "本质来自上下文/对象/流不敏感的保守合流",
        ], RED),
        bullet_box(10, 8.85, 2.75, 3.7, 3.15, "诊断解释", [
            "safe 输出可视为 must-safe，当前未发现错误",
            "must-unsafe 输出与真实必失败场景一致",
            "may-unsafe 是保守告警，包含真实风险和假阳性",
            "因此优化时可优先消费 safe 与 must-unsafe",
        ], GREEN),
        footer(11),
    ]
    slides.append((slide_xml(shapes), False))

    return slides


def write_static_files(root: Path, slide_count: int) -> None:
    (root / "_rels").mkdir(parents=True)
    (root / "docProps").mkdir()
    (root / "ppt" / "_rels").mkdir(parents=True)
    (root / "ppt" / "slides" / "_rels").mkdir(parents=True)
    (root / "ppt" / "slideMasters" / "_rels").mkdir(parents=True)
    (root / "ppt" / "slideLayouts" / "_rels").mkdir(parents=True)
    (root / "ppt" / "theme").mkdir(parents=True)
    (root / "ppt" / "media").mkdir(parents=True)

    slide_overrides = "\n".join(
        f'<Override PartName="/ppt/slides/slide{i}.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>'
        for i in range(1, slide_count + 1)
    )
    (root / "[Content_Types].xml").write_text(f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Default Extension="png" ContentType="image/png"/>
  <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
  <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
  <Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
  <Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
  {slide_overrides}
</Types>
""", encoding="utf-8")

    (root / "_rels" / ".rels").write_text("""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>
""", encoding="utf-8")

    (root / "docProps" / "core.xml").write_text("""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
 xmlns:dc="http://purl.org/dc/elements/1.1/"
 xmlns:dcterms="http://purl.org/dc/terms/"
 xmlns:dcmitype="http://purl.org/dc/dcmitype/"
 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dc:title>RCPTA 项目汇报</dc:title>
  <dc:creator>Codex</dc:creator>
  <cp:lastModifiedBy>Codex</cp:lastModifiedBy>
  <dcterms:created xsi:type="dcterms:W3CDTF">2026-07-01T00:00:00Z</dcterms:created>
  <dcterms:modified xsi:type="dcterms:W3CDTF">2026-07-01T00:00:00Z</dcterms:modified>
</cp:coreProperties>
""", encoding="utf-8")

    (root / "docProps" / "app.xml").write_text(f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"
 xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
  <Application>Codex PPTX Generator</Application>
  <PresentationFormat>On-screen Show (16:9)</PresentationFormat>
  <Slides>{slide_count}</Slides>
</Properties>
""", encoding="utf-8")

    slide_ids = "\n".join(
        f'<p:sldId id="{255 + i}" r:id="rId{i}"/>' for i in range(1, slide_count + 1)
    )
    (root / "ppt" / "presentation.xml").write_text(f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
 xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
 xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId{slide_count + 1}"/></p:sldMasterIdLst>
  <p:sldIdLst>{slide_ids}</p:sldIdLst>
  <p:sldSz cx="{W}" cy="{H}" type="wide"/>
  <p:notesSz cx="6858000" cy="9144000"/>
  <p:defaultTextStyle/>
</p:presentation>
""", encoding="utf-8")

    rel_entries = "\n".join(
        f'<Relationship Id="rId{i}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide{i}.xml"/>'
        for i in range(1, slide_count + 1)
    )
    rel_entries += f'\n<Relationship Id="rId{slide_count + 1}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>'
    rel_entries += f'\n<Relationship Id="rId{slide_count + 2}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>'
    (root / "ppt" / "_rels" / "presentation.xml.rels").write_text(f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">{rel_entries}</Relationships>
""", encoding="utf-8")

    (root / "ppt" / "slideMasters" / "slideMaster1.xml").write_text(f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
 xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
 xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="{W}" cy="{H}"/><a:chOff x="0" y="0"/><a:chExt cx="{W}" cy="{H}"/></a:xfrm></p:grpSpPr></p:spTree></p:cSld>
  <p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
  <p:sldLayoutIdLst><p:sldLayoutId id="2147483649" r:id="rId1"/></p:sldLayoutIdLst>
  <p:txStyles><p:titleStyle/><p:bodyStyle/><p:otherStyle/></p:txStyles>
</p:sldMaster>
""", encoding="utf-8")

    (root / "ppt" / "slideMasters" / "_rels" / "slideMaster1.xml.rels").write_text("""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme1.xml"/>
</Relationships>
""", encoding="utf-8")

    (root / "ppt" / "slideLayouts" / "slideLayout1.xml").write_text(f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
 xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
 xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="blank" preserve="1">
  <p:cSld name="Blank"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="{W}" cy="{H}"/><a:chOff x="0" y="0"/><a:chExt cx="{W}" cy="{H}"/></a:xfrm></p:grpSpPr></p:spTree></p:cSld>
  <p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>
</p:sldLayout>
""", encoding="utf-8")

    (root / "ppt" / "slideLayouts" / "_rels" / "slideLayout1.xml.rels").write_text("""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>
</Relationships>
""", encoding="utf-8")

    (root / "ppt" / "theme" / "theme1.xml").write_text(f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="RCPTA">
  <a:themeElements>
    <a:clrScheme name="RCPTA">
      <a:dk1><a:srgbClr val="000000"/></a:dk1><a:lt1><a:srgbClr val="FFFFFF"/></a:lt1>
      <a:dk2><a:srgbClr val="323232"/></a:dk2><a:lt2><a:srgbClr val="E3DED1"/></a:lt2>
      <a:accent1><a:srgbClr val="{ORANGE}"/></a:accent1><a:accent2><a:srgbClr val="9F2936"/></a:accent2>
      <a:accent3><a:srgbClr val="1B587C"/></a:accent3><a:accent4><a:srgbClr val="4E8542"/></a:accent4>
      <a:accent5><a:srgbClr val="604878"/></a:accent5><a:accent6><a:srgbClr val="C19859"/></a:accent6>
      <a:hlink><a:srgbClr val="6B9F25"/></a:hlink><a:folHlink><a:srgbClr val="B26B02"/></a:folHlink>
    </a:clrScheme>
    <a:fontScheme name="RCPTA Fonts"><a:majorFont><a:latin typeface="{TITLE_FONT}"/><a:ea typeface="{TITLE_FONT}"/><a:cs typeface="{TITLE_FONT}"/></a:majorFont><a:minorFont><a:latin typeface="{FONT}"/><a:ea typeface="{FONT}"/><a:cs typeface="{FONT}"/></a:minorFont></a:fontScheme>
    <a:fmtScheme name="RCPTA"><a:fillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:fillStyleLst><a:lnStyleLst><a:ln w="9525"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln></a:lnStyleLst><a:effectStyleLst><a:effectStyle><a:effectLst/></a:effectStyle></a:effectStyleLst><a:bgFillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:bgFillStyleLst></a:fmtScheme>
  </a:themeElements>
  <a:objectDefaults/><a:extraClrSchemeLst/>
</a:theme>
""", encoding="utf-8")


def main() -> None:
    if TMP.exists():
        shutil.rmtree(TMP)
    TMP.mkdir(parents=True)
    slides = make_slides()
    write_static_files(TMP, len(slides))

    shutil.copyfile(IMG, TMP / "ppt" / "media" / "rcpta_arc_v3.png")
    shutil.copyfile(SEG_LOGO, TMP / "ppt" / "media" / "template_seg.png")
    shutil.copyfile(NJU_LOGO, TMP / "ppt" / "media" / "template_nju.png")
    for i, (xml, has_image) in enumerate(slides, start=1):
        (TMP / "ppt" / "slides" / f"slide{i}.xml").write_text(xml, encoding="utf-8")
        (TMP / "ppt" / "slides" / "_rels" / f"slide{i}.xml.rels").write_text(rels_xml(has_image), encoding="utf-8")

    if OUT.exists():
        OUT.unlink()
    with zipfile.ZipFile(OUT, "w", compression=zipfile.ZIP_DEFLATED) as zf:
        for path in sorted(TMP.rglob("*")):
            if path.is_file():
                zf.write(path, path.relative_to(TMP).as_posix())
    print(f"wrote {OUT} ({len(slides)} slides)")


if __name__ == "__main__":
    main()
