#!/usr/bin/env python3
from __future__ import annotations

import html
import re
import shutil
import zipfile
from pathlib import Path


SRC = Path("doc/rcpta_project_report_speech_script.md")
OUT = Path("doc/rcpta_project_report_speech_script.docx")
TMP = Path("/tmp/rcpta_speech_docx")


def esc(text: str) -> str:
    return html.escape(text, quote=True)


def text_runs(text: str) -> str:
    parts = re.split(r"(`[^`]+`)", text)
    runs = []
    for part in parts:
        if not part:
            continue
        if part.startswith("`") and part.endswith("`"):
            runs.append(
                f'<w:r><w:rPr><w:rFonts w:ascii="Consolas" w:eastAsia="Consolas" w:hAnsi="Consolas"/>'
                f'<w:sz w:val="21"/></w:rPr><w:t>{esc(part[1:-1])}</w:t></w:r>'
            )
        else:
            runs.append(f"<w:r><w:t>{esc(part)}</w:t></w:r>")
    return "".join(runs)


def para(text: str = "", style: str | None = None) -> str:
    ppr = f"<w:pPr><w:pStyle w:val=\"{style}\"/></w:pPr>" if style else ""
    return f"<w:p>{ppr}{text_runs(text)}</w:p>"


def bullet(text: str) -> str:
    return (
        '<w:p><w:pPr><w:pStyle w:val="ListParagraph"/>'
        '<w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>'
        f"{text_runs(text)}</w:p>"
    )


def md_to_document_xml(md: str) -> str:
    body = []
    for raw in md.splitlines():
        line = raw.rstrip()
        if not line:
            body.append(para())
            continue
        if line.startswith("# "):
            body.append(para(line[2:], "Title"))
        elif line.startswith("## "):
            body.append(para(line[3:], "Heading1"))
        elif line.startswith("### "):
            body.append(para(line[4:], "Heading2"))
        elif line.startswith("- "):
            body.append(bullet(line[2:]))
        else:
            body.append(para(line))
    sect = (
        '<w:sectPr>'
        '<w:pgSz w:w="11906" w:h="16838"/>'
        '<w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440" w:header="720" w:footer="720" w:gutter="0"/>'
        '<w:cols w:space="720"/><w:docGrid w:linePitch="312"/>'
        '</w:sectPr>'
    )
    return f'''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:wpc="http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas"
 xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
 xmlns:o="urn:schemas-microsoft-com:office:office"
 xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
 xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"
 xmlns:v="urn:schemas-microsoft-com:vml"
 xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing"
 xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"
 xmlns:w10="urn:schemas-microsoft-com:office:word"
 xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
 xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"
 xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup"
 xmlns:wpi="http://schemas.microsoft.com/office/word/2010/wordprocessingInk"
 xmlns:wne="http://schemas.microsoft.com/office/word/2006/wordml"
 xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"
 mc:Ignorable="w14 wp14">
 <w:body>
  {''.join(body)}
  {sect}
 </w:body>
</w:document>
'''


def styles_xml() -> str:
    return '''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
 <w:docDefaults>
  <w:rPrDefault><w:rPr><w:rFonts w:ascii="Arial" w:eastAsia="宋体" w:hAnsi="Arial"/><w:sz w:val="24"/><w:szCs w:val="24"/></w:rPr></w:rPrDefault>
  <w:pPrDefault><w:pPr><w:spacing w:after="160" w:line="360" w:lineRule="auto"/></w:pPr></w:pPrDefault>
 </w:docDefaults>
 <w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/><w:qFormat/></w:style>
 <w:style w:type="paragraph" w:styleId="Title">
  <w:name w:val="Title"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/>
  <w:pPr><w:jc w:val="center"/><w:spacing w:after="360"/></w:pPr>
  <w:rPr><w:rFonts w:ascii="Arial Black" w:eastAsia="黑体" w:hAnsi="Arial Black"/><w:b/><w:sz w:val="36"/></w:rPr>
 </w:style>
 <w:style w:type="paragraph" w:styleId="Heading1">
  <w:name w:val="heading 1"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/>
  <w:pPr><w:keepNext/><w:spacing w:before="280" w:after="160"/><w:outlineLvl w:val="0"/></w:pPr>
  <w:rPr><w:rFonts w:ascii="Arial" w:eastAsia="黑体" w:hAnsi="Arial"/><w:b/><w:color w:val="604878"/><w:sz w:val="30"/></w:rPr>
 </w:style>
 <w:style w:type="paragraph" w:styleId="Heading2">
  <w:name w:val="heading 2"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/><w:qFormat/>
  <w:pPr><w:keepNext/><w:spacing w:before="180" w:after="100"/><w:outlineLvl w:val="1"/></w:pPr>
  <w:rPr><w:rFonts w:ascii="Arial" w:eastAsia="黑体" w:hAnsi="Arial"/><w:b/><w:color w:val="222222"/><w:sz w:val="26"/></w:rPr>
 </w:style>
 <w:style w:type="paragraph" w:styleId="ListParagraph">
  <w:name w:val="List Paragraph"/><w:basedOn w:val="Normal"/><w:qFormat/>
  <w:pPr><w:ind w:left="720" w:hanging="360"/></w:pPr>
 </w:style>
</w:styles>
'''


def numbering_xml() -> str:
    return '''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
 <w:abstractNum w:abstractNumId="0">
  <w:multiLevelType w:val="hybridMultilevel"/>
  <w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="bullet"/><w:lvlText w:val="•"/><w:lvlJc w:val="left"/><w:pPr><w:ind w:left="720" w:hanging="360"/></w:pPr></w:lvl>
 </w:abstractNum>
 <w:num w:numId="1"><w:abstractNumId w:val="0"/></w:num>
</w:numbering>
'''


def write_docx() -> None:
    if TMP.exists():
        shutil.rmtree(TMP)
    (TMP / "_rels").mkdir(parents=True)
    (TMP / "docProps").mkdir()
    (TMP / "word" / "_rels").mkdir(parents=True)

    (TMP / "[Content_Types].xml").write_text('''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
 <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
 <Default Extension="xml" ContentType="application/xml"/>
 <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
 <Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>
 <Override PartName="/word/numbering.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"/>
 <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
 <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>
''', encoding="utf-8")
    (TMP / "_rels" / ".rels").write_text('''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
 <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
 <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
 <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>
''', encoding="utf-8")
    (TMP / "word" / "_rels" / "document.xml.rels").write_text('''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
 <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
 <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering" Target="numbering.xml"/>
</Relationships>
''', encoding="utf-8")
    (TMP / "docProps" / "core.xml").write_text('''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
 xmlns:dc="http://purl.org/dc/elements/1.1/"
 xmlns:dcterms="http://purl.org/dc/terms/"
 xmlns:dcmitype="http://purl.org/dc/dcmitype/"
 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
 <dc:title>RCPTA 项目汇报讲稿</dc:title>
 <dc:creator>Codex</dc:creator>
 <cp:lastModifiedBy>Codex</cp:lastModifiedBy>
 <dcterms:created xsi:type="dcterms:W3CDTF">2026-07-01T00:00:00Z</dcterms:created>
 <dcterms:modified xsi:type="dcterms:W3CDTF">2026-07-01T00:00:00Z</dcterms:modified>
</cp:coreProperties>
''', encoding="utf-8")
    (TMP / "docProps" / "app.xml").write_text('''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"
 xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
 <Application>Codex DOCX Generator</Application>
</Properties>
''', encoding="utf-8")
    (TMP / "word" / "document.xml").write_text(md_to_document_xml(SRC.read_text(encoding="utf-8")), encoding="utf-8")
    (TMP / "word" / "styles.xml").write_text(styles_xml(), encoding="utf-8")
    (TMP / "word" / "numbering.xml").write_text(numbering_xml(), encoding="utf-8")

    if OUT.exists():
        OUT.unlink()
    with zipfile.ZipFile(OUT, "w", compression=zipfile.ZIP_DEFLATED) as zf:
        for p in sorted(TMP.rglob("*")):
            if p.is_file():
                zf.write(p, p.relative_to(TMP).as_posix())
    print(f"wrote {OUT}")


if __name__ == "__main__":
    write_docx()
