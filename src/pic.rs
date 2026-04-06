use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EdcAdjustpoint {
    #[serde(rename = "@_addr")]
    pub _addr: String,
    #[serde(rename = "@_begin")]
    pub _begin: String,
    #[serde(rename = "@_end")]
    pub _end: String,
    #[serde(rename = "@_modsrc")]
    pub _modsrc: String,
    #[serde(rename = "@_refcount")]
    pub _refcount: String,
    #[serde(rename = "@edc:_addr")]
    pub edc__addr: String,
    #[serde(rename = "@edc:_begin")]
    pub edc__begin: String,
    #[serde(rename = "@edc:_end")]
    pub edc__end: String,
    #[serde(rename = "@edc:_modsrc")]
    pub edc__modsrc: String,
    #[serde(rename = "@edc:_refcount")]
    pub edc__refcount: String,
    #[serde(rename = "@edc:offset")]
    pub edc_offset: String,
    #[serde(rename = "@edc:unimplval")]
    pub edc_unimplval: String,
    #[serde(rename = "@offset")]
    pub offset: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcAliaslist {
    #[serde(rename = "edc:HITECHAlias", default)]
    pub edc__h_i_t_e_c_h_alias: Vec<EdcHitechalias>,
    #[serde(rename = "edc:LegacyAlias", default)]
    pub edc__legacy_alias: Vec<EdcLegacyalias>,
    #[serde(rename = "edc:MigrationAlias", default)]
    pub edc__migration_alias: Vec<EdcMigrationalias>,
}

#[derive(Debug, Deserialize)]
pub struct EdcAltconfigfusesector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:kseg0")]
    pub edc_kseg0: String,
    #[serde(rename = "@edc:kseg1")]
    pub edc_kseg1: String,
    #[serde(rename = "@edc:ksegdef")]
    pub edc_ksegdef: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:regionidref")]
    pub edc_regionidref: String,
    #[serde(rename = "@edc:registerprefix")]
    pub edc_registerprefix: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:DCRDef", default)]
    pub edc__d_c_r_def: Vec<EdcDcrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcAltvectorarea {
    #[serde(rename = "@edc:nzsize")]
    pub edc_nzsize: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@nzsize")]
    pub nzsize: String,
    #[serde(rename = "@regionid")]
    pub regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcAltvectorsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcArchdef {
    #[serde(rename = "@edc:archgroup")]
    pub edc_archgroup: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:name")]
    pub edc_name: String,
    #[serde(rename = "edc:MemTraits")]
    pub edc__mem_traits: EdcMemtraits,
}

#[derive(Debug, Deserialize)]
pub struct EdcAuxcodesector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcAuxresetsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcAuxvectorsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcBackbugvectorsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcBackgrounddebugmemtraits {
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcBootconfigmemtraits {
    #[serde(rename = "@edc:addrinc")]
    pub edc_addrinc: String,
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcBootconfigsector {
    #[serde(rename = "@atdf:pagesize")]
    pub atdf_pagesize: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:isotp")]
    pub edc_isotp: String,
    #[serde(rename = "@edc:iswritable")]
    pub edc_iswritable: String,
    #[serde(rename = "@edc:kseg0")]
    pub edc_kseg0: String,
    #[serde(rename = "@edc:kseg1")]
    pub edc_kseg1: String,
    #[serde(rename = "@edc:ksegdef")]
    pub edc_ksegdef: String,
    #[serde(rename = "@edc:kuseg")]
    pub edc_kuseg: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcBootsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcBreakpoints {
    #[serde(rename = "@edc:debugmoduletype")]
    pub edc_debugmoduletype: String,
    #[serde(rename = "@edc:hasappinout")]
    pub edc_hasappinout: String,
    #[serde(rename = "@edc:hasdatacapture")]
    pub edc_hasdatacapture: String,
    #[serde(rename = "@edc:hwbpcount")]
    pub edc_hwbpcount: String,
    #[serde(rename = "@edc:idbyte")]
    pub edc_idbyte: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcCaldatamemtraits {
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcCaldatazone {
    #[serde(rename = "@atdf:pagesize")]
    pub atdf_pagesize: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:issection")]
    pub edc_issection: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
    #[serde(rename = "@edc:sectiondesc")]
    pub edc_sectiondesc: String,
    #[serde(rename = "@edc:sectionname")]
    pub edc_sectionname: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:DCRDef", default)]
    pub edc__d_c_r_def: Vec<EdcDcrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcChecksum {
    #[serde(rename = "@beginaddr")]
    pub beginaddr: String,
    #[serde(rename = "@checksumalgo")]
    pub checksumalgo: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:checksumalgo")]
    pub edc_checksumalgo: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@endaddr")]
    pub endaddr: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcCodeguard {
    #[serde(rename = "@edc:securityclass")]
    pub edc_securityclass: String,
    #[serde(rename = "edc:SegRegion", default)]
    pub edc__seg_region: Vec<EdcSegregion>,
}

#[derive(Debug, Deserialize)]
pub struct EdcCodememtraits {
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcCodesector {
    #[serde(rename = "@atdf:pagesize")]
    pub atdf_pagesize: String,
    #[serde(rename = "@beginaddr")]
    pub beginaddr: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:isprogrammedatruntime")]
    pub edc_isprogrammedatruntime: String,
    #[serde(rename = "@edc:isreserved")]
    pub edc_isreserved: String,
    #[serde(rename = "@edc:issection")]
    pub edc_issection: String,
    #[serde(rename = "@edc:kseg0")]
    pub edc_kseg0: String,
    #[serde(rename = "@edc:kseg1")]
    pub edc_kseg1: String,
    #[serde(rename = "@edc:ksegdef")]
    pub edc_ksegdef: String,
    #[serde(rename = "@edc:kuseg")]
    pub edc_kuseg: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
    #[serde(rename = "@edc:sectiondesc")]
    pub edc_sectiondesc: String,
    #[serde(rename = "@edc:sectionname")]
    pub edc_sectionname: String,
    #[serde(rename = "@endaddr")]
    pub endaddr: String,
    #[serde(rename = "@regionid")]
    pub regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcConfigfusememtraits {
    #[serde(rename = "@edc:addrinc")]
    pub edc_addrinc: String,
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:unimplval")]
    pub edc_unimplval: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcConfigfuseotpmemtraits {
    #[serde(rename = "@edc:addrinc")]
    pub edc_addrinc: String,
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcConfigfusesector {
    #[serde(rename = "@atdf:pagesize")]
    pub atdf_pagesize: String,
    #[serde(rename = "@beginaddr")]
    pub beginaddr: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:erasepagesize")]
    pub edc_erasepagesize: String,
    #[serde(rename = "@edc:isotp")]
    pub edc_isotp: String,
    #[serde(rename = "@edc:kseg0")]
    pub edc_kseg0: String,
    #[serde(rename = "@edc:kseg1")]
    pub edc_kseg1: String,
    #[serde(rename = "@edc:ksegdef")]
    pub edc_ksegdef: String,
    #[serde(rename = "@edc:magicoffset")]
    pub edc_magicoffset: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
    #[serde(rename = "@endaddr")]
    pub endaddr: String,
    #[serde(rename = "@regionid")]
    pub regionid: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:DCRDef", default)]
    pub edc__d_c_r_def: Vec<EdcDcrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcConfigwormmemtraits {
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcConfigwormsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcDcisector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "edc:Register", default)]
    pub edc__register: Vec<EdcRegister>,
}

#[derive(Debug, Deserialize)]
pub struct EdcDcrdef {
    #[serde(rename = "@_addr")]
    pub _addr: String,
    #[serde(rename = "@_begin")]
    pub _begin: String,
    #[serde(rename = "@_end")]
    pub _end: String,
    #[serde(rename = "@_modsrc")]
    pub _modsrc: String,
    #[serde(rename = "@_refcount")]
    pub _refcount: String,
    #[serde(rename = "@access")]
    pub access: String,
    #[serde(rename = "@cname")]
    pub cname: String,
    #[serde(rename = "@default")]
    pub default: String,
    #[serde(rename = "@desc")]
    pub desc: String,
    #[serde(rename = "@edc:_addr")]
    pub edc__addr: String,
    #[serde(rename = "@edc:_begin")]
    pub edc__begin: String,
    #[serde(rename = "@edc:_end")]
    pub edc__end: String,
    #[serde(rename = "@edc:_modsrc")]
    pub edc__modsrc: String,
    #[serde(rename = "@edc:_refcount")]
    pub edc__refcount: String,
    #[serde(rename = "@edc:access")]
    pub edc_access: String,
    #[serde(rename = "@edc:backwardcompatibleinit")]
    pub edc_backwardcompatibleinit: String,
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:default")]
    pub edc_default: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:factorydefault")]
    pub edc_factorydefault: String,
    #[serde(rename = "@edc:impl")]
    pub edc_impl: String,
    #[serde(rename = "@edc:ishidden")]
    pub edc_ishidden: String,
    #[serde(rename = "@edc:isidehidden")]
    pub edc_isidehidden: String,
    #[serde(rename = "@edc:islanghidden")]
    pub edc_islanghidden: String,
    #[serde(rename = "@edc:name")]
    pub edc_name: String,
    #[serde(rename = "@edc:nzwidth")]
    pub edc_nzwidth: String,
    #[serde(rename = "@edc:unimplval")]
    pub edc_unimplval: String,
    #[serde(rename = "@edc:unused")]
    pub edc_unused: String,
    #[serde(rename = "@edc:useinchecksum")]
    pub edc_useinchecksum: String,
    #[serde(rename = "@factorydefault")]
    pub factorydefault: String,
    #[serde(rename = "@impl")]
    pub impl_: String,
    #[serde(rename = "@ishidden")]
    pub ishidden: String,
    #[serde(rename = "@isidehidden")]
    pub isidehidden: String,
    #[serde(rename = "@ltx:memberofperipheral")]
    pub ltx_memberofperipheral: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@nzwidth")]
    pub nzwidth: String,
    #[serde(rename = "@unimplval")]
    pub unimplval: String,
    #[serde(rename = "@unused")]
    pub unused: String,
    #[serde(rename = "@useinchecksum")]
    pub useinchecksum: String,
    #[serde(rename = "edc:AliasList", default)]
    pub edc__alias_list: Vec<EdcAliaslist>,
    #[serde(rename = "edc:DCRModeList", default)]
    pub edc__d_c_r_mode_list: Vec<EdcDcrmodelist>,
    #[serde(rename = "edc:Illegal", default)]
    pub edc__illegal: Vec<EdcIllegal>,
}

#[derive(Debug, Deserialize)]
pub struct EdcDcrfielddef {
    #[serde(rename = "@cname")]
    pub cname: String,
    #[serde(rename = "@desc")]
    pub desc: String,
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:ishidden")]
    pub edc_ishidden: String,
    #[serde(rename = "@edc:isidehidden")]
    pub edc_isidehidden: String,
    #[serde(rename = "@edc:islanghidden")]
    pub edc_islanghidden: String,
    #[serde(rename = "@edc:mask")]
    pub edc_mask: String,
    #[serde(rename = "@edc:name")]
    pub edc_name: String,
    #[serde(rename = "@edc:nzwidth")]
    pub edc_nzwidth: String,
    #[serde(rename = "@edc:uuid")]
    pub edc_uuid: String,
    #[serde(rename = "@ishidden")]
    pub ishidden: String,
    #[serde(rename = "@isidehidden")]
    pub isidehidden: String,
    #[serde(rename = "@islanghidden")]
    pub islanghidden: String,
    #[serde(rename = "@mask")]
    pub mask: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@nzwidth")]
    pub nzwidth: String,
    #[serde(rename = "edc:AliasList", default)]
    pub edc__alias_list: Vec<EdcAliaslist>,
    #[serde(rename = "edc:DCRFieldSemantic", default)]
    pub edc__d_c_r_field_semantic: Vec<EdcDcrfieldsemantic>,
    #[serde(rename = "edc:DCRFieldValueRange", default)]
    pub edc__d_c_r_field_value_range: Vec<EdcDcrfieldvaluerange>,
}

#[derive(Debug, Deserialize)]
pub struct EdcDcrfieldsemantic {
    #[serde(rename = "@cname")]
    pub cname: String,
    #[serde(rename = "@codeguardidref")]
    pub codeguardidref: String,
    #[serde(rename = "@desc")]
    pub desc: String,
    #[serde(rename = "@edc:_defeatcoercion")]
    pub edc__defeatcoercion: String,
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:codeguardidref")]
    pub edc_codeguardidref: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:ishidden")]
    pub edc_ishidden: String,
    #[serde(rename = "@edc:isidehidden")]
    pub edc_isidehidden: String,
    #[serde(rename = "@edc:islanghidden")]
    pub edc_islanghidden: String,
    #[serde(rename = "@edc:memmodeidref")]
    pub edc_memmodeidref: String,
    #[serde(rename = "@edc:oscmodeidref")]
    pub edc_oscmodeidref: String,
    #[serde(rename = "@edc:when")]
    pub edc_when: String,
    #[serde(rename = "@ishidden")]
    pub ishidden: String,
    #[serde(rename = "@when")]
    pub when: String,
    #[serde(rename = "edc:AliasList", default)]
    pub edc__alias_list: Vec<EdcAliaslist>,
    #[serde(rename = "edc:Checksum", default)]
    pub edc__checksum: Vec<EdcChecksum>,
}

#[derive(Debug, Deserialize)]
pub struct EdcDcrfieldvaluerange {
    #[serde(rename = "@cname")]
    pub cname: String,
    #[serde(rename = "@desc")]
    pub desc: String,
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:max")]
    pub edc_max: String,
    #[serde(rename = "@edc:min")]
    pub edc_min: String,
    #[serde(rename = "@max")]
    pub max: String,
    #[serde(rename = "@min")]
    pub min: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcDcrmode {
    #[serde(rename = "@edc:id")]
    pub edc_id: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@ltx:memberofperipheral")]
    pub ltx_memberofperipheral: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:DCRFieldDef", default)]
    pub edc__d_c_r_field_def: Vec<EdcDcrfielddef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcDcrmodelist {
    #[serde(rename = "edc:DCRMode", default)]
    pub edc__d_c_r_mode: Vec<EdcDcrmode>,
}

#[derive(Debug, Deserialize)]
pub struct EdcDdrsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:kseg0")]
    pub edc_kseg0: String,
    #[serde(rename = "@edc:kseg1")]
    pub edc_kseg1: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcDevidtorev {
    #[serde(rename = "@edc:revlist")]
    pub edc_revlist: String,
    #[serde(rename = "@edc:value")]
    pub edc_value: String,
    #[serde(rename = "@revlist")]
    pub revlist: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcDiasector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "edc:RegisterArray", default)]
    pub edc__register_array: Vec<EdcRegisterarray>,
}

#[derive(Debug, Deserialize)]
pub struct EdcDma {
    #[serde(rename = "@edc:dmaflavor")]
    pub edc_dmaflavor: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcDmaspace {
    #[serde(rename = "edc:SFRDataSector", default)]
    pub edc__s_f_r_data_sector: Vec<EdcSfrdatasector>,
}

#[derive(Debug, Deserialize)]
pub struct EdcDprdatasector {
    #[serde(rename = "@edc:bank")]
    pub edc_bank: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:shadowidref")]
    pub edc_shadowidref: String,
    #[serde(rename = "@edc:shadowoffset")]
    pub edc_shadowoffset: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:SFRDef", default)]
    pub edc__s_f_r_def: Vec<EdcSfrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcDprsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:SFRDef", default)]
    pub edc__s_f_r_def: Vec<EdcSfrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcDataflashspace {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:magicoffset")]
    pub edc_magicoffset: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcDatamemtraits {
    #[serde(rename = "@edc:addrinc")]
    pub edc_addrinc: String,
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcDataspace {
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:magicoffset")]
    pub edc_magicoffset: String,
    #[serde(rename = "@edc:xbeginaddr")]
    pub edc_xbeginaddr: String,
    #[serde(rename = "@edc:xendaddr")]
    pub edc_xendaddr: String,
    #[serde(rename = "@edc:ybeginaddr")]
    pub edc_ybeginaddr: String,
    #[serde(rename = "@edc:yendaddr")]
    pub edc_yendaddr: String,
    #[serde(rename = "edc:ExtendedModeOnly")]
    pub edc__extended_mode_only: EdcExtendedmodeonly,
    #[serde(rename = "edc:RegardlessOfMode")]
    pub edc__regardless_of_mode: EdcRegardlessofmode,
    #[serde(rename = "edc:RegisterMap", default)]
    pub edc__register_map: Vec<EdcRegistermap>,
    #[serde(rename = "edc:TraditionalModeOnly")]
    pub edc__traditional_mode_only: EdcTraditionalmodeonly,
}

#[derive(Debug, Deserialize)]
pub struct EdcDeviceidmemtraits {
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcDeviceidsector {
    #[serde(rename = "@atdf:pagesize")]
    pub atdf_pagesize: String,
    #[serde(rename = "@edc:_mask")]
    pub edc__mask: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:issection")]
    pub edc_issection: String,
    #[serde(rename = "@edc:kseg0")]
    pub edc_kseg0: String,
    #[serde(rename = "@edc:kseg1")]
    pub edc_kseg1: String,
    #[serde(rename = "@edc:ksegdef")]
    pub edc_ksegdef: String,
    #[serde(rename = "@edc:mask")]
    pub edc_mask: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
    #[serde(rename = "@edc:sectiondesc")]
    pub edc_sectiondesc: String,
    #[serde(rename = "@edc:sectionname")]
    pub edc_sectionname: String,
    #[serde(rename = "@edc:useinchecksum")]
    pub edc_useinchecksum: String,
    #[serde(rename = "@edc:value")]
    pub edc_value: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:DCRDef", default)]
    pub edc__d_c_r_def: Vec<EdcDcrdef>,
    #[serde(rename = "edc:DEVIDToRev", default)]
    pub edc__d_e_v_i_d_to_rev: Vec<EdcDevidtorev>,
}

#[derive(Debug, Deserialize)]
pub struct EdcEbidatasector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:kseg2")]
    pub edc_kseg2: String,
    #[serde(rename = "@edc:kseg3")]
    pub edc_kseg3: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcEdswindowsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcEedatamemtraits {
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:magicoffset")]
    pub edc_magicoffset: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcEedatasector {
    #[serde(rename = "@atdf:pagesize")]
    pub atdf_pagesize: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:issection")]
    pub edc_issection: String,
    #[serde(rename = "@edc:magicoffset")]
    pub edc_magicoffset: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
    #[serde(rename = "@edc:sectiondesc")]
    pub edc_sectiondesc: String,
    #[serde(rename = "@edc:sectionname")]
    pub edc_sectionname: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcEmulatorsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcEmulatorzone {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcExtcodememtraits {
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcExtcodesector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcExtendeddataspace {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcExtendedmodeonly {
    #[serde(rename = "edc:GPRDataSector", default)]
    pub edc__g_p_r_data_sector: Vec<EdcGprdatasector>,
}

#[derive(Debug, Deserialize)]
pub struct EdcExternalmemorymode {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:id")]
    pub edc_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcExternalsector {
    #[serde(rename = "@edc:accessprotocol")]
    pub edc_accessprotocol: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:isexecutable")]
    pub edc_isexecutable: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
    #[serde(rename = "@edc:type")]
    pub edc_type: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcFpu {
}

#[derive(Debug, Deserialize)]
pub struct EdcFlashdatasector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:issection")]
    pub edc_issection: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:sectiondesc")]
    pub edc_sectiondesc: String,
    #[serde(rename = "@edc:sectionname")]
    pub edc_sectionname: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcFreeze {
    #[serde(rename = "@edc:addr")]
    pub edc_addr: String,
    #[serde(rename = "edc:FreezeBit")]
    pub edc__freeze_bit: EdcFreezebit,
}

#[derive(Debug, Deserialize)]
pub struct EdcFreezebit {
    #[serde(rename = "@edc:freezeval")]
    pub edc_freezeval: String,
    #[serde(rename = "@edc:name")]
    pub edc_name: String,
    #[serde(rename = "@edc:pos")]
    pub edc_pos: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcFusesspace {
    #[serde(rename = "@edc:magicoffset")]
    pub edc_magicoffset: String,
    #[serde(rename = "edc:ConfigFuseSector")]
    pub edc__config_fuse_sector: EdcConfigfusesector,
}

#[derive(Debug, Deserialize)]
pub struct EdcGprdatasector {
    #[serde(rename = "@atdf:pagesize")]
    pub atdf_pagesize: String,
    #[serde(rename = "@edc:bank")]
    pub edc_bank: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:isexecutable")]
    pub edc_isexecutable: String,
    #[serde(rename = "@edc:isprogrammedatruntime")]
    pub edc_isprogrammedatruntime: String,
    #[serde(rename = "@edc:kseg0")]
    pub edc_kseg0: String,
    #[serde(rename = "@edc:kseg1")]
    pub edc_kseg1: String,
    #[serde(rename = "@edc:ksegdef")]
    pub edc_ksegdef: String,
    #[serde(rename = "@edc:kuseg")]
    pub edc_kuseg: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
    #[serde(rename = "@edc:shadowidref")]
    pub edc_shadowidref: String,
    #[serde(rename = "@edc:shadowoffset")]
    pub edc_shadowoffset: String,
    #[serde(rename = "@edc:virtualprogramoffset")]
    pub edc_virtualprogramoffset: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:SFRDef", default)]
    pub edc__s_f_r_def: Vec<EdcSfrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcHitechalias {
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:propkey")]
    pub edc_propkey: String,
    #[serde(rename = "@edc:propvalue")]
    pub edc_propvalue: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcIcspwriteinhibitsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcIllegal {
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:when")]
    pub edc_when: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcImport {
}

#[derive(Debug, Deserialize)]
pub struct EdcIndirectspace {
    #[serde(rename = "edc:LinearDataSector")]
    pub edc__linear_data_sector: EdcLineardatasector,
}

#[derive(Debug, Deserialize)]
pub struct EdcInstructionset {
    #[serde(rename = "@edc:instructionsetid")]
    pub edc_instructionsetid: String,
    #[serde(rename = "edc:InstructionSet", default)]
    pub edc__instruction_set: Vec<EdcInstructionset>,
}

#[derive(Debug, Deserialize)]
pub struct EdcInterrupt {
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:irq")]
    pub edc_irq: String,
    #[serde(rename = "@ltx:memberofperipheral")]
    pub ltx_memberofperipheral: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcInterruptlist {
    #[serde(rename = "@edc:hasvariableoffsets")]
    pub edc_hasvariableoffsets: String,
    #[serde(rename = "@edc:interruptcontrollerflavor")]
    pub edc_interruptcontrollerflavor: String,
    #[serde(rename = "@edc:shadowsetcount")]
    pub edc_shadowsetcount: String,
    #[serde(rename = "@ltx:defaultbaseaddr")]
    pub ltx_defaultbaseaddr: String,
    #[serde(rename = "edc:Interrupt", default)]
    pub edc__interrupt: Vec<EdcInterrupt>,
    #[serde(rename = "edc:InterruptRequest", default)]
    pub edc__interrupt_request: Vec<EdcInterruptrequest>,
    #[serde(rename = "edc:Trap", default)]
    pub edc__trap: Vec<EdcTrap>,
}

#[derive(Debug, Deserialize)]
pub struct EdcInterruptrequest {
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:irq")]
    pub edc_irq: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcJoinedsfrdef {
    #[serde(rename = "@edc:_addr")]
    pub edc__addr: String,
    #[serde(rename = "@edc:_begin")]
    pub edc__begin: String,
    #[serde(rename = "@edc:_end")]
    pub edc__end: String,
    #[serde(rename = "@edc:_modsrc")]
    pub edc__modsrc: String,
    #[serde(rename = "@edc:_refcount")]
    pub edc__refcount: String,
    #[serde(rename = "@edc:access")]
    pub edc_access: String,
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:impl")]
    pub edc_impl: String,
    #[serde(rename = "@edc:islanghidden")]
    pub edc_islanghidden: String,
    #[serde(rename = "@edc:name")]
    pub edc_name: String,
    #[serde(rename = "@edc:nzwidth")]
    pub edc_nzwidth: String,
    #[serde(rename = "@ltx:memberofperipheral")]
    pub ltx_memberofperipheral: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:AliasList", default)]
    pub edc__alias_list: Vec<EdcAliaslist>,
    #[serde(rename = "edc:MuxedSFRDef", default)]
    pub edc__muxed_s_f_r_def: Vec<EdcMuxedsfrdef>,
    #[serde(rename = "edc:SFRDef", default)]
    pub edc__s_f_r_def: Vec<EdcSfrdef>,
    #[serde(rename = "edc:SFRModeList", default)]
    pub edc__s_f_r_mode_list: Vec<EdcSfrmodelist>,
    #[serde(rename = "edc:StimInfo", default)]
    pub edc__stim_info: Vec<EdcStiminfo>,
}

#[derive(Debug, Deserialize)]
pub struct EdcL1cache {
}

#[derive(Debug, Deserialize)]
pub struct EdcLcd {
    #[serde(rename = "@edc:segcount")]
    pub edc_segcount: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcLatches {
    #[serde(rename = "@edc:bootcfg")]
    pub edc_bootcfg: String,
    #[serde(rename = "@edc:cfg")]
    pub edc_cfg: String,
    #[serde(rename = "@edc:eedata")]
    pub edc_eedata: String,
    #[serde(rename = "@edc:flashdata")]
    pub edc_flashdata: String,
    #[serde(rename = "@edc:pgm")]
    pub edc_pgm: String,
    #[serde(rename = "@edc:rowerase")]
    pub edc_rowerase: String,
    #[serde(rename = "@edc:userid")]
    pub edc_userid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcLegacyalias {
    #[serde(rename = "@cname")]
    pub cname: String,
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:propkey")]
    pub edc_propkey: String,
    #[serde(rename = "@propkey")]
    pub propkey: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcLineardatasector {
    #[serde(rename = "@edc:banksize")]
    pub edc_banksize: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:blockbeginaddr")]
    pub edc_blockbeginaddr: String,
    #[serde(rename = "@edc:blockendaddr")]
    pub edc_blockendaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcLockbitsspace {
    #[serde(rename = "@edc:magicoffset")]
    pub edc_magicoffset: String,
    #[serde(rename = "edc:ConfigFuseSector")]
    pub edc__config_fuse_sector: EdcConfigfusesector,
}

#[derive(Debug, Deserialize)]
pub struct EdcMemtraits {
    #[serde(rename = "@edc:addrinc")]
    pub edc_addrinc: String,
    #[serde(rename = "@edc:bankcount")]
    pub edc_bankcount: String,
    #[serde(rename = "@edc:extendeddataspace")]
    pub edc_extendeddataspace: String,
    #[serde(rename = "@edc:hwstackdepth")]
    pub edc_hwstackdepth: String,
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
    #[serde(rename = "edc:BackgroundDebugMemTraits")]
    pub edc__background_debug_mem_traits: EdcBackgrounddebugmemtraits,
    #[serde(rename = "edc:BootConfigMemTraits")]
    pub edc__boot_config_mem_traits: EdcBootconfigmemtraits,
    #[serde(rename = "edc:CalDataMemTraits")]
    pub edc__cal_data_mem_traits: EdcCaldatamemtraits,
    #[serde(rename = "edc:CodeMemTraits")]
    pub edc__code_mem_traits: EdcCodememtraits,
    #[serde(rename = "edc:ConfigFuseMemTraits")]
    pub edc__config_fuse_mem_traits: EdcConfigfusememtraits,
    #[serde(rename = "edc:ConfigFuseOTPMemTraits")]
    pub edc__config_fuse_o_t_p_mem_traits: EdcConfigfuseotpmemtraits,
    #[serde(rename = "edc:ConfigWORMMemTraits")]
    pub edc__config_w_o_r_m_mem_traits: EdcConfigwormmemtraits,
    #[serde(rename = "edc:DataMemTraits")]
    pub edc__data_mem_traits: EdcDatamemtraits,
    #[serde(rename = "edc:DeviceIDMemTraits")]
    pub edc__device_i_d_mem_traits: EdcDeviceidmemtraits,
    #[serde(rename = "edc:EEDataMemTraits")]
    pub edc__e_e_data_mem_traits: EdcEedatamemtraits,
    #[serde(rename = "edc:ExtCodeMemTraits")]
    pub edc__ext_code_mem_traits: EdcExtcodememtraits,
    #[serde(rename = "edc:NMMRMemTraits")]
    pub edc__n_m_m_r_mem_traits: EdcNmmrmemtraits,
    #[serde(rename = "edc:ProgramMemTraits")]
    pub edc__program_mem_traits: EdcProgrammemtraits,
    #[serde(rename = "edc:TestMemTraits")]
    pub edc__test_mem_traits: EdcTestmemtraits,
    #[serde(rename = "edc:UniqueIDMemTraits", default)]
    pub edc__unique_i_d_mem_traits: Vec<EdcUniqueidmemtraits>,
    #[serde(rename = "edc:UserIDMemTraits")]
    pub edc__user_i_d_mem_traits: EdcUseridmemtraits,
    #[serde(rename = "edc:UserOTPMemTraits")]
    pub edc__user_o_t_p_mem_traits: EdcUserotpmemtraits,
}

#[derive(Debug, Deserialize)]
pub struct EdcMemorymodelist {
    #[serde(rename = "@edc:addr")]
    pub edc_addr: String,
    #[serde(rename = "edc:ExternalMemoryMode", default)]
    pub edc__external_memory_mode: Vec<EdcExternalmemorymode>,
    #[serde(rename = "edc:MicrocontrollerMode")]
    pub edc__microcontroller_mode: EdcMicrocontrollermode,
}

#[derive(Debug, Deserialize)]
pub struct EdcMicrocontrollermode {
    #[serde(rename = "@edc:id")]
    pub edc_id: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcMigrationalias {
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcMirror {
    #[serde(rename = "@edc:_addr")]
    pub edc__addr: String,
    #[serde(rename = "@edc:nzsize")]
    pub edc_nzsize: String,
    #[serde(rename = "@edc:regionidref")]
    pub edc_regionidref: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcMuxedsfrdef {
    #[serde(rename = "@edc:_addr")]
    pub edc__addr: String,
    #[serde(rename = "@edc:_begin")]
    pub edc__begin: String,
    #[serde(rename = "@edc:_end")]
    pub edc__end: String,
    #[serde(rename = "@edc:_modsrc")]
    pub edc__modsrc: String,
    #[serde(rename = "@edc:_refcount")]
    pub edc__refcount: String,
    #[serde(rename = "@edc:addr")]
    pub edc_addr: String,
    #[serde(rename = "@edc:nzwidth")]
    pub edc_nzwidth: String,
    #[serde(rename = "@ltx:memberofperipheral")]
    pub ltx_memberofperipheral: String,
    #[serde(rename = "edc:SelectSFR", default)]
    pub edc__select_s_f_r: Vec<EdcSelectsfr>,
}

#[derive(Debug, Deserialize)]
pub struct EdcNmmrmemtraits {
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcNmmrplace {
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "edc:JoinedSFRDef", default)]
    pub edc__joined_s_f_r_def: Vec<EdcJoinedsfrdef>,
    #[serde(rename = "edc:SFRDef", default)]
    pub edc__s_f_r_def: Vec<EdcSfrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcOscillator {
    #[serde(rename = "edc:OscillatorMode", default)]
    pub edc__oscillator_mode: Vec<EdcOscillatormode>,
}

#[derive(Debug, Deserialize)]
pub struct EdcOscillatormode {
    #[serde(rename = "@edc:id")]
    pub edc_id: String,
    #[serde(rename = "@edc:max")]
    pub edc_max: String,
    #[serde(rename = "@edc:min")]
    pub edc_min: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcPic {
    #[serde(rename = "@atdf:architecture")]
    pub atdf_architecture: String,
    #[serde(rename = "@atdf:family")]
    pub atdf_family: String,
    #[serde(rename = "@edc:acname")]
    pub edc_acname: String,
    #[serde(rename = "@edc:arch")]
    pub edc_arch: String,
    #[serde(rename = "@edc:atsamname")]
    pub edc_atsamname: String,
    #[serde(rename = "@edc:clonedfrom")]
    pub edc_clonedfrom: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:dosid")]
    pub edc_dosid: String,
    #[serde(rename = "@edc:dsid")]
    pub edc_dsid: String,
    #[serde(rename = "@edc:hasFreeze")]
    pub edc_has_freeze: String,
    #[serde(rename = "@edc:hasfreeze")]
    pub edc_hasfreeze: String,
    #[serde(rename = "@edc:informedby")]
    pub edc_informedby: String,
    #[serde(rename = "@edc:isdebuggable")]
    pub edc_isdebuggable: String,
    #[serde(rename = "@edc:isextended")]
    pub edc_isextended: String,
    #[serde(rename = "@edc:isslaveof")]
    pub edc_isslaveof: String,
    #[serde(rename = "@edc:masksetid")]
    pub edc_masksetid: String,
    #[serde(rename = "@edc:name")]
    pub edc_name: String,
    #[serde(rename = "@edc:procid")]
    pub edc_procid: String,
    #[serde(rename = "@edc:psid")]
    pub edc_psid: String,
    #[serde(rename = "@xsi:schemaLocation")]
    pub xsi_schema_location: String,
    #[serde(rename = "edc:ArchDef")]
    pub edc__arch_def: EdcArchdef,
    #[serde(rename = "edc:Breakpoints")]
    pub edc__breakpoints: EdcBreakpoints,
    #[serde(rename = "edc:CodeGuard")]
    pub edc__code_guard: EdcCodeguard,
    #[serde(rename = "edc:DMA", default)]
    pub edc__d_m_a: Vec<EdcDma>,
    #[serde(rename = "edc:DMASpace")]
    pub edc__d_m_a_space: EdcDmaspace,
    #[serde(rename = "edc:DataFlashSpace")]
    pub edc__data_flash_space: EdcDataflashspace,
    #[serde(rename = "edc:DataSpace")]
    pub edc__data_space: EdcDataspace,
    #[serde(rename = "edc:ExtendedDataSpace")]
    pub edc__extended_data_space: EdcExtendeddataspace,
    #[serde(rename = "edc:FPU")]
    pub edc__f_p_u: EdcFpu,
    #[serde(rename = "edc:Freeze")]
    pub edc__freeze: EdcFreeze,
    #[serde(rename = "edc:FusesSpace")]
    pub edc__fuses_space: EdcFusesspace,
    #[serde(rename = "edc:Import")]
    pub edc__import: EdcImport,
    #[serde(rename = "edc:IndirectSpace")]
    pub edc__indirect_space: EdcIndirectspace,
    #[serde(rename = "edc:InstructionSet")]
    pub edc__instruction_set: EdcInstructionset,
    #[serde(rename = "edc:InterruptList")]
    pub edc__interrupt_list: EdcInterruptlist,
    #[serde(rename = "edc:L1Cache")]
    pub edc__l1_cache: EdcL1cache,
    #[serde(rename = "edc:LCD")]
    pub edc__l_c_d: EdcLcd,
    #[serde(rename = "edc:LockbitsSpace")]
    pub edc__lockbits_space: EdcLockbitsspace,
    #[serde(rename = "edc:MemoryModeList")]
    pub edc__memory_mode_list: EdcMemorymodelist,
    #[serde(rename = "edc:Oscillator")]
    pub edc__oscillator: EdcOscillator,
    #[serde(rename = "edc:PeripheralList")]
    pub edc__peripheral_list: EdcPeripherallist,
    #[serde(rename = "edc:PhysicalSpace")]
    pub edc__physical_space: EdcPhysicalspace,
    #[serde(rename = "edc:PinList", default)]
    pub edc__pin_list: Vec<EdcPinlist>,
    #[serde(rename = "edc:Power")]
    pub edc__power: EdcPower,
    #[serde(rename = "edc:ProdSignaturesSpace")]
    pub edc__prod_signatures_space: EdcProdsignaturesspace,
    #[serde(rename = "edc:ProgramSpace")]
    pub edc__program_space: EdcProgramspace,
    #[serde(rename = "edc:Programming")]
    pub edc__programming: EdcProgramming,
    #[serde(rename = "edc:ProjectionTable")]
    pub edc__projection_table: EdcProjectiontable,
    #[serde(rename = "edc:Properties")]
    pub edc__properties: EdcProperties,
    #[serde(rename = "edc:SignaturesSpace")]
    pub edc__signatures_space: EdcSignaturesspace,
    #[serde(rename = "edc:SlaveCores")]
    pub edc__slave_cores: EdcSlavecores,
    #[serde(rename = "edc:UserSignaturesSpace")]
    pub edc__user_signatures_space: EdcUsersignaturesspace,
    #[serde(rename = "edc:WatchdogTimer")]
    pub edc__watchdog_timer: EdcWatchdogtimer,
}

#[derive(Debug, Deserialize)]
pub struct EdcPeripheral {
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcPeripherallist {
    #[serde(rename = "edc:Peripheral", default)]
    pub edc__peripheral: Vec<EdcPeripheral>,
}

#[derive(Debug, Deserialize)]
pub struct EdcPhysicalspace {
    #[serde(rename = "edc:AltConfigFuseSector", default)]
    pub edc__alt_config_fuse_sector: Vec<EdcAltconfigfusesector>,
    #[serde(rename = "edc:BootConfigSector", default)]
    pub edc__boot_config_sector: Vec<EdcBootconfigsector>,
    #[serde(rename = "edc:CalDataZone")]
    pub edc__cal_data_zone: EdcCaldatazone,
    #[serde(rename = "edc:CodeSector", default)]
    pub edc__code_sector: Vec<EdcCodesector>,
    #[serde(rename = "edc:ConfigFuseSector", default)]
    pub edc__config_fuse_sector: Vec<EdcConfigfusesector>,
    #[serde(rename = "edc:DDRSector")]
    pub edc__d_d_r_sector: EdcDdrsector,
    #[serde(rename = "edc:DPRSector", default)]
    pub edc__d_p_r_sector: Vec<EdcDprsector>,
    #[serde(rename = "edc:DeviceIDSector")]
    pub edc__device_i_d_sector: EdcDeviceidsector,
    #[serde(rename = "edc:EBIDataSector")]
    pub edc__e_b_i_data_sector: EdcEbidatasector,
    #[serde(rename = "edc:EmulatorSector")]
    pub edc__emulator_sector: EdcEmulatorsector,
    #[serde(rename = "edc:ExternalSector", default)]
    pub edc__external_sector: Vec<EdcExternalsector>,
    #[serde(rename = "edc:GPRDataSector", default)]
    pub edc__g_p_r_data_sector: Vec<EdcGprdatasector>,
    #[serde(rename = "edc:NMMRPlace")]
    pub edc__n_m_m_r_place: EdcNmmrplace,
    #[serde(rename = "edc:RegisterMap", default)]
    pub edc__register_map: Vec<EdcRegistermap>,
    #[serde(rename = "edc:SFRDataSector", default)]
    pub edc__s_f_r_data_sector: Vec<EdcSfrdatasector>,
    #[serde(rename = "edc:SPIFlashSector")]
    pub edc__s_p_i_flash_sector: EdcSpiflashsector,
    #[serde(rename = "edc:SQIDataSector")]
    pub edc__s_q_i_data_sector: EdcSqidatasector,
    #[serde(rename = "edc:TestZone")]
    pub edc__test_zone: EdcTestzone,
    #[serde(rename = "edc:UserIDSector", default)]
    pub edc__user_i_d_sector: Vec<EdcUseridsector>,
    #[serde(rename = "edc:UserZone", default)]
    pub edc__user_zone: Vec<EdcUserzone>,
}

#[derive(Debug, Deserialize)]
pub struct EdcPin {
    #[serde(rename = "edc:VirtualPin", default)]
    pub edc__virtual_pin: Vec<EdcVirtualpin>,
}

#[derive(Debug, Deserialize)]
pub struct EdcPinlist {
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:ppsflavor")]
    pub edc_ppsflavor: String,
    #[serde(rename = "@xsi:schemaLocation")]
    pub xsi_schema_location: String,
    #[serde(rename = "edc:Pin", default)]
    pub edc__pin: Vec<EdcPin>,
    #[serde(rename = "edc:RemappablePin", default)]
    pub edc__remappable_pin: Vec<EdcRemappablepin>,
}

#[derive(Debug, Deserialize)]
pub struct EdcPower {
    #[serde(rename = "@edc:hashighvoltagemclr")]
    pub edc_hashighvoltagemclr: String,
    #[serde(rename = "@edc:hashighvoltagemclr2")]
    pub edc_hashighvoltagemclr2: String,
    #[serde(rename = "@edc:sharespowersupply")]
    pub edc_sharespowersupply: String,
    #[serde(rename = "edc:VDD")]
    pub edc__v_d_d: EdcVdd,
    #[serde(rename = "edc:VPP")]
    pub edc__v_p_p: EdcVpp,
}

#[derive(Debug, Deserialize)]
pub struct EdcProdsignaturesspace {
    #[serde(rename = "edc:CalDataZone")]
    pub edc__cal_data_zone: EdcCaldatazone,
}

#[derive(Debug, Deserialize)]
pub struct EdcProgrammemtraits {
    #[serde(rename = "@edc:addrinc")]
    pub edc_addrinc: String,
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcProgramspace {
    #[serde(rename = "edc:AltVectorArea")]
    pub edc__alt_vector_area: EdcAltvectorarea,
    #[serde(rename = "edc:AltVectorSector")]
    pub edc__alt_vector_sector: EdcAltvectorsector,
    #[serde(rename = "edc:AuxCodeSector")]
    pub edc__aux_code_sector: EdcAuxcodesector,
    #[serde(rename = "edc:AuxResetSector")]
    pub edc__aux_reset_sector: EdcAuxresetsector,
    #[serde(rename = "edc:AuxVectorSector")]
    pub edc__aux_vector_sector: EdcAuxvectorsector,
    #[serde(rename = "edc:BACKBUGVectorSector")]
    pub edc__b_a_c_k_b_u_g_vector_sector: EdcBackbugvectorsector,
    #[serde(rename = "edc:BootSector")]
    pub edc__boot_sector: EdcBootsector,
    #[serde(rename = "edc:CalDataZone", default)]
    pub edc__cal_data_zone: Vec<EdcCaldatazone>,
    #[serde(rename = "edc:CodeSector", default)]
    pub edc__code_sector: Vec<EdcCodesector>,
    #[serde(rename = "edc:ConfigFuseSector", default)]
    pub edc__config_fuse_sector: Vec<EdcConfigfusesector>,
    #[serde(rename = "edc:ConfigWORMSector")]
    pub edc__config_w_o_r_m_sector: EdcConfigwormsector,
    #[serde(rename = "edc:DCISector")]
    pub edc__d_c_i_sector: EdcDcisector,
    #[serde(rename = "edc:DIASector")]
    pub edc__d_i_a_sector: EdcDiasector,
    #[serde(rename = "edc:DeviceIDSector")]
    pub edc__device_i_d_sector: EdcDeviceidsector,
    #[serde(rename = "edc:EEDataSector", default)]
    pub edc__e_e_data_sector: Vec<EdcEedatasector>,
    #[serde(rename = "edc:ExtCodeSector")]
    pub edc__ext_code_sector: EdcExtcodesector,
    #[serde(rename = "edc:FlashDataSector")]
    pub edc__flash_data_sector: EdcFlashdatasector,
    #[serde(rename = "edc:ICSPWriteInhibitSector", default)]
    pub edc__i_c_s_p_write_inhibit_sector: Vec<EdcIcspwriteinhibitsector>,
    #[serde(rename = "edc:ProgramSubspace", default)]
    pub edc__program_subspace: Vec<EdcProgramsubspace>,
    #[serde(rename = "edc:ResetSector")]
    pub edc__reset_sector: EdcResetsector,
    #[serde(rename = "edc:RevisionIDSector")]
    pub edc__revision_i_d_sector: EdcRevisionidsector,
    #[serde(rename = "edc:TestZone", default)]
    pub edc__test_zone: Vec<EdcTestzone>,
    #[serde(rename = "edc:UniqueIDSector")]
    pub edc__unique_i_d_sector: EdcUniqueidsector,
    #[serde(rename = "edc:UserIDSector")]
    pub edc__user_i_d_sector: EdcUseridsector,
    #[serde(rename = "edc:UserOTPSector")]
    pub edc__user_o_t_p_sector: EdcUserotpsector,
    #[serde(rename = "edc:VectorArea")]
    pub edc__vector_area: EdcVectorarea,
    #[serde(rename = "edc:VectorSector")]
    pub edc__vector_sector: EdcVectorsector,
    #[serde(rename = "edc:WORMHoleSector", default)]
    pub edc__w_o_r_m_hole_sector: Vec<EdcWormholesector>,
}

#[derive(Debug, Deserialize)]
pub struct EdcProgramsubspace {
    #[serde(rename = "@beginaddr")]
    pub beginaddr: String,
    #[serde(rename = "@endaddr")]
    pub endaddr: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@partitionmode")]
    pub partitionmode: String,
    #[serde(rename = "edc:AltVectorArea", default)]
    pub edc__alt_vector_area: Vec<EdcAltvectorarea>,
    #[serde(rename = "edc:CodeSector", default)]
    pub edc__code_sector: Vec<EdcCodesector>,
    #[serde(rename = "edc:ConfigFuseSector", default)]
    pub edc__config_fuse_sector: Vec<EdcConfigfusesector>,
    #[serde(rename = "edc:ResetSector", default)]
    pub edc__reset_sector: Vec<EdcResetsector>,
    #[serde(rename = "edc:VectorSector", default)]
    pub edc__vector_sector: Vec<EdcVectorsector>,
    #[serde(rename = "edc:WORMHoleSector", default)]
    pub edc__w_o_r_m_hole_sector: Vec<EdcWormholesector>,
}

#[derive(Debug, Deserialize)]
pub struct EdcProgramming {
    #[serde(rename = "@edc:arraysize")]
    pub edc_arraysize: String,
    #[serde(rename = "@edc:boundary")]
    pub edc_boundary: String,
    #[serde(rename = "@edc:csumadd")]
    pub edc_csumadd: String,
    #[serde(rename = "@edc:erasealgo")]
    pub edc_erasealgo: String,
    #[serde(rename = "@edc:erasepagesize")]
    pub edc_erasepagesize: String,
    #[serde(rename = "@edc:haschecksum")]
    pub edc_haschecksum: String,
    #[serde(rename = "@edc:haslvp")]
    pub edc_haslvp: String,
    #[serde(rename = "@edc:haslvp2")]
    pub edc_haslvp2: String,
    #[serde(rename = "@edc:hasrowerasecmd")]
    pub edc_hasrowerasecmd: String,
    #[serde(rename = "@edc:hasvppfirst")]
    pub edc_hasvppfirst: String,
    #[serde(rename = "@edc:lvpthresh")]
    pub edc_lvpthresh: String,
    #[serde(rename = "@edc:maskalgo")]
    pub edc_maskalgo: String,
    #[serde(rename = "@edc:memtech")]
    pub edc_memtech: String,
    #[serde(rename = "@edc:ovrpgm")]
    pub edc_ovrpgm: String,
    #[serde(rename = "@edc:pagesize")]
    pub edc_pagesize: String,
    #[serde(rename = "@edc:panelsize")]
    pub edc_panelsize: String,
    #[serde(rename = "@edc:pkgthresh")]
    pub edc_pkgthresh: String,
    #[serde(rename = "@edc:protocol")]
    pub edc_protocol: String,
    #[serde(rename = "@edc:tries")]
    pub edc_tries: String,
    #[serde(rename = "edc:Latches")]
    pub edc__latches: EdcLatches,
    #[serde(rename = "edc:ProgrammingRowSize", default)]
    pub edc__programming_row_size: Vec<EdcProgrammingrowsize>,
    #[serde(rename = "edc:ProgrammingWaitTime", default)]
    pub edc__programming_wait_time: Vec<EdcProgrammingwaittime>,
    #[serde(rename = "edc:SpeedRanges")]
    pub edc__speed_ranges: EdcSpeedranges,
    #[serde(rename = "edc:Wait")]
    pub edc__wait: EdcWait,
}

#[derive(Debug, Deserialize)]
pub struct EdcProgrammingrowsize {
    #[serde(rename = "@edc:addressablesize")]
    pub edc_addressablesize: String,
    #[serde(rename = "@edc:nzsize")]
    pub edc_nzsize: String,
    #[serde(rename = "@edc:progop")]
    pub edc_progop: String,
    #[serde(rename = "@edc:sizeunits")]
    pub edc_sizeunits: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcProgrammingwaittime {
    #[serde(rename = "@edc:progop")]
    pub edc_progop: String,
    #[serde(rename = "@edc:time")]
    pub edc_time: String,
    #[serde(rename = "@edc:timeunits")]
    pub edc_timeunits: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcProjection {
    #[serde(rename = "@edc:ksegval")]
    pub edc_ksegval: String,
    #[serde(rename = "@edc:ptovop")]
    pub edc_ptovop: String,
    #[serde(rename = "@edc:ptovval")]
    pub edc_ptovval: String,
    #[serde(rename = "@edc:vtopop")]
    pub edc_vtopop: String,
    #[serde(rename = "@edc:vtopval")]
    pub edc_vtopval: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcProjectiontable {
    #[serde(rename = "@edc:ksegmask")]
    pub edc_ksegmask: String,
    #[serde(rename = "edc:Projection", default)]
    pub edc__projection: Vec<EdcProjection>,
}

#[derive(Debug, Deserialize)]
pub struct EdcProperties {
    #[serde(rename = "edc:Property", default)]
    pub edc__property: Vec<EdcProperty>,
}

#[derive(Debug, Deserialize)]
pub struct EdcProperty {
    #[serde(rename = "@edc:propkey")]
    pub edc_propkey: String,
    #[serde(rename = "@edc:propvalue")]
    pub edc_propvalue: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcRegardlessofmode {
    #[serde(rename = "edc:ConfigFuseSector", default)]
    pub edc__config_fuse_sector: Vec<EdcConfigfusesector>,
    #[serde(rename = "edc:DPRDataSector", default)]
    pub edc__d_p_r_data_sector: Vec<EdcDprdatasector>,
    #[serde(rename = "edc:DeviceIDSector", default)]
    pub edc__device_i_d_sector: Vec<EdcDeviceidsector>,
    #[serde(rename = "edc:EDSWindowSector")]
    pub edc__e_d_s_window_sector: EdcEdswindowsector,
    #[serde(rename = "edc:EEDataSector")]
    pub edc__e_e_data_sector: EdcEedatasector,
    #[serde(rename = "edc:EmulatorSector")]
    pub edc__emulator_sector: EdcEmulatorsector,
    #[serde(rename = "edc:EmulatorZone")]
    pub edc__emulator_zone: EdcEmulatorzone,
    #[serde(rename = "edc:ExternalSector")]
    pub edc__external_sector: EdcExternalsector,
    #[serde(rename = "edc:GPRDataSector", default)]
    pub edc__g_p_r_data_sector: Vec<EdcGprdatasector>,
    #[serde(rename = "edc:NMMRPlace")]
    pub edc__n_m_m_r_place: EdcNmmrplace,
    #[serde(rename = "edc:SFRDataSector", default)]
    pub edc__s_f_r_data_sector: Vec<EdcSfrdatasector>,
    #[serde(rename = "edc:SystemGPRDataSector", default)]
    pub edc__system_g_p_r_data_sector: Vec<EdcSystemgprdatasector>,
    #[serde(rename = "edc:UserIDSector", default)]
    pub edc__user_i_d_sector: Vec<EdcUseridsector>,
}

#[derive(Debug, Deserialize)]
pub struct EdcRegister {
    #[serde(rename = "@edc:_addr")]
    pub edc__addr: String,
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:nzwidth")]
    pub edc_nzwidth: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcRegisterarray {
    #[serde(rename = "@edc:_addr")]
    pub edc__addr: String,
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "edc:Register", default)]
    pub edc__register: Vec<EdcRegister>,
}

#[derive(Debug, Deserialize)]
pub struct EdcRegistermap {
    #[serde(rename = "@edc:id")]
    pub edc_id: String,
    #[serde(rename = "edc:MuxedSFRDef", default)]
    pub edc__muxed_s_f_r_def: Vec<EdcMuxedsfrdef>,
    #[serde(rename = "edc:SFRDef", default)]
    pub edc__s_f_r_def: Vec<EdcSfrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcRegistermirror {
    #[serde(rename = "@edc:cnameref")]
    pub edc_cnameref: String,
    #[serde(rename = "@edc:cnamesuffix")]
    pub edc_cnamesuffix: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcRemappablepin {
    #[serde(rename = "@edc:direction")]
    pub edc_direction: String,
    #[serde(rename = "@edc:ppsports")]
    pub edc_ppsports: String,
    #[serde(rename = "edc:VirtualPin", default)]
    pub edc__virtual_pin: Vec<EdcVirtualpin>,
}

#[derive(Debug, Deserialize)]
pub struct EdcResetsector {
    #[serde(rename = "@beginaddr")]
    pub beginaddr: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@endaddr")]
    pub endaddr: String,
    #[serde(rename = "@regionid")]
    pub regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcRevisionidsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "edc:DEVIDToRev", default)]
    pub edc__d_e_v_i_d_to_rev: Vec<EdcDevidtorev>,
}

#[derive(Debug, Deserialize)]
pub struct EdcSfrdatasector {
    #[serde(rename = "@edc:alignment")]
    pub edc_alignment: String,
    #[serde(rename = "@edc:bank")]
    pub edc_bank: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:isotp")]
    pub edc_isotp: String,
    #[serde(rename = "@edc:kseg0")]
    pub edc_kseg0: String,
    #[serde(rename = "@edc:kseg1")]
    pub edc_kseg1: String,
    #[serde(rename = "@edc:ksegdef")]
    pub edc_ksegdef: String,
    #[serde(rename = "@edc:kuseg")]
    pub edc_kuseg: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
    #[serde(rename = "@edc:shadowidref")]
    pub edc_shadowidref: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:JoinedSFRDef", default)]
    pub edc__joined_s_f_r_def: Vec<EdcJoinedsfrdef>,
    #[serde(rename = "edc:Mirror", default)]
    pub edc__mirror: Vec<EdcMirror>,
    #[serde(rename = "edc:MuxedSFRDef", default)]
    pub edc__muxed_s_f_r_def: Vec<EdcMuxedsfrdef>,
    #[serde(rename = "edc:RegisterMirror", default)]
    pub edc__register_mirror: Vec<EdcRegistermirror>,
    #[serde(rename = "edc:SFRDef", default)]
    pub edc__s_f_r_def: Vec<EdcSfrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcSfrdef {
    #[serde(rename = "@None:baseofperipheral")]
    pub none_baseofperipheral: String,
    #[serde(rename = "@None:memberofperipheral")]
    pub none_memberofperipheral: String,
    #[serde(rename = "@edc:_addr")]
    pub edc__addr: String,
    #[serde(rename = "@edc:_begin")]
    pub edc__begin: String,
    #[serde(rename = "@edc:_end")]
    pub edc__end: String,
    #[serde(rename = "@edc:_modsrc")]
    pub edc__modsrc: String,
    #[serde(rename = "@edc:_refcount")]
    pub edc__refcount: String,
    #[serde(rename = "@edc:access")]
    pub edc_access: String,
    #[serde(rename = "@edc:addr")]
    pub edc_addr: String,
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:designname")]
    pub edc_designname: String,
    #[serde(rename = "@edc:dma")]
    pub edc_dma: String,
    #[serde(rename = "@edc:grp")]
    pub edc_grp: String,
    #[serde(rename = "@edc:impl")]
    pub edc_impl: String,
    #[serde(rename = "@edc:ishidden")]
    pub edc_ishidden: String,
    #[serde(rename = "@edc:isidehidden")]
    pub edc_isidehidden: String,
    #[serde(rename = "@edc:isindirect")]
    pub edc_isindirect: String,
    #[serde(rename = "@edc:islanghidden")]
    pub edc_islanghidden: String,
    #[serde(rename = "@edc:isvolatile")]
    pub edc_isvolatile: String,
    #[serde(rename = "@edc:mclr")]
    pub edc_mclr: String,
    #[serde(rename = "@edc:name")]
    pub edc_name: String,
    #[serde(rename = "@edc:nmmrflags")]
    pub edc_nmmrflags: String,
    #[serde(rename = "@edc:nmmrid")]
    pub edc_nmmrid: String,
    #[serde(rename = "@edc:nzlangwidth")]
    pub edc_nzlangwidth: String,
    #[serde(rename = "@edc:nzwidth")]
    pub edc_nzwidth: String,
    #[serde(rename = "@edc:por")]
    pub edc_por: String,
    #[serde(rename = "@edc:portals")]
    pub edc_portals: String,
    #[serde(rename = "@ltx:baseofperipheral")]
    pub ltx_baseofperipheral: String,
    #[serde(rename = "@ltx:memberofperipheral")]
    pub ltx_memberofperipheral: String,
    #[serde(rename = "edc:AliasList", default)]
    pub edc__alias_list: Vec<EdcAliaslist>,
    #[serde(rename = "edc:FreezeBit", default)]
    pub edc__freeze_bit: Vec<EdcFreezebit>,
    #[serde(rename = "edc:SFRModeList", default)]
    pub edc__s_f_r_mode_list: Vec<EdcSfrmodelist>,
    #[serde(rename = "edc:StimInfo", default)]
    pub edc__stim_info: Vec<EdcStiminfo>,
}

#[derive(Debug, Deserialize)]
pub struct EdcSfrfielddef {
    #[serde(rename = "@edc:aspect")]
    pub edc_aspect: String,
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:ishidden")]
    pub edc_ishidden: String,
    #[serde(rename = "@edc:isidehidden")]
    pub edc_isidehidden: String,
    #[serde(rename = "@edc:islanghidden")]
    pub edc_islanghidden: String,
    #[serde(rename = "@edc:mask")]
    pub edc_mask: String,
    #[serde(rename = "@edc:name")]
    pub edc_name: String,
    #[serde(rename = "@edc:nzwidth")]
    pub edc_nzwidth: String,
    #[serde(rename = "edc:SFRFieldSemantic", default)]
    pub edc__s_f_r_field_semantic: Vec<EdcSfrfieldsemantic>,
}

#[derive(Debug, Deserialize)]
pub struct EdcSfrfieldsemantic {
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
    #[serde(rename = "@edc:when")]
    pub edc_when: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcSfrmode {
    #[serde(rename = "@None:memberofperipheral")]
    pub none_memberofperipheral: String,
    #[serde(rename = "@edc:access")]
    pub edc_access: String,
    #[serde(rename = "@edc:id")]
    pub edc_id: String,
    #[serde(rename = "@edc:mclr")]
    pub edc_mclr: String,
    #[serde(rename = "@edc:por")]
    pub edc_por: String,
    #[serde(rename = "@ltx:memberofperipheral")]
    pub ltx_memberofperipheral: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:SFRFieldDef", default)]
    pub edc__s_f_r_field_def: Vec<EdcSfrfielddef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcSfrmodelist {
    #[serde(rename = "edc:SFRMode", default)]
    pub edc__s_f_r_mode: Vec<EdcSfrmode>,
}

#[derive(Debug, Deserialize)]
pub struct EdcSpiflashsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:kseg0")]
    pub edc_kseg0: String,
    #[serde(rename = "@edc:kseg1")]
    pub edc_kseg1: String,
    #[serde(rename = "@edc:ksegdef")]
    pub edc_ksegdef: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcSqidatasector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:kseg2")]
    pub edc_kseg2: String,
    #[serde(rename = "@edc:kseg3")]
    pub edc_kseg3: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcSeg {
    #[serde(rename = "@edc:seg")]
    pub edc_seg: String,
    #[serde(rename = "edc:SegNature", default)]
    pub edc__seg_nature: Vec<EdcSegnature>,
    #[serde(rename = "edc:SegWriteProtect", default)]
    pub edc__seg_write_protect: Vec<EdcSegwriteprotect>,
}

#[derive(Debug, Deserialize)]
pub struct EdcSegnature {
    #[serde(rename = "@edc:addr")]
    pub edc_addr: String,
    #[serde(rename = "@edc:id")]
    pub edc_id: String,
    #[serde(rename = "@edc:segsecurity")]
    pub edc_segsecurity: String,
    #[serde(rename = "@edc:segsize")]
    pub edc_segsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcSegregion {
    #[serde(rename = "@edc:segregion")]
    pub edc_segregion: String,
    #[serde(rename = "edc:Seg", default)]
    pub edc__seg: Vec<EdcSeg>,
}

#[derive(Debug, Deserialize)]
pub struct EdcSegwriteprotect {
    #[serde(rename = "@edc:disableid")]
    pub edc_disableid: String,
    #[serde(rename = "@edc:enableid")]
    pub edc_enableid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcSelectsfr {
    #[serde(rename = "@edc:_begin")]
    pub edc__begin: String,
    #[serde(rename = "@edc:_end")]
    pub edc__end: String,
    #[serde(rename = "@edc:_modsrc")]
    pub edc__modsrc: String,
    #[serde(rename = "@edc:_refcount")]
    pub edc__refcount: String,
    #[serde(rename = "@edc:when")]
    pub edc_when: String,
    #[serde(rename = "edc:SFRDef", default)]
    pub edc__s_f_r_def: Vec<EdcSfrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcSignaturesspace {
    #[serde(rename = "@edc:magicoffset")]
    pub edc_magicoffset: String,
    #[serde(rename = "edc:DeviceIDSector")]
    pub edc__device_i_d_sector: EdcDeviceidsector,
}

#[derive(Debug, Deserialize)]
pub struct EdcSlavecore {
    #[serde(rename = "@edc:name")]
    pub edc_name: String,
    #[serde(rename = "@edc:src")]
    pub edc_src: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcSlavecores {
    #[serde(rename = "edc:SlaveCore")]
    pub edc__slave_core: EdcSlavecore,
}

#[derive(Debug, Deserialize)]
pub struct EdcSpeedranges {
    #[serde(rename = "@edc:highvoltagespeed")]
    pub edc_highvoltagespeed: String,
    #[serde(rename = "@edc:lowvoltagespeed")]
    pub edc_lowvoltagespeed: String,
    #[serde(rename = "@edc:mediumvoltagespeed")]
    pub edc_mediumvoltagespeed: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcStiminfo {
    #[serde(rename = "@edc:stimpcfiles")]
    pub edc_stimpcfiles: String,
    #[serde(rename = "@edc:stimregfiles")]
    pub edc_stimregfiles: String,
    #[serde(rename = "@edc:stimscl")]
    pub edc_stimscl: String,
    #[serde(rename = "@edc:stimtype")]
    pub edc_stimtype: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcSystemgprdatasector {
    #[serde(rename = "@edc:bank")]
    pub edc_bank: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcTestmemtraits {
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcTestzone {
    #[serde(rename = "@edc:appbeginaddr")]
    pub edc_appbeginaddr: String,
    #[serde(rename = "@edc:appendaddr")]
    pub edc_appendaddr: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcTraditionalmodeonly {
    #[serde(rename = "edc:GPRDataSector")]
    pub edc__g_p_r_data_sector: EdcGprdatasector,
}

#[derive(Debug, Deserialize)]
pub struct EdcTrap {
    #[serde(rename = "@edc:cname")]
    pub edc_cname: String,
    #[serde(rename = "@edc:desc")]
    pub edc_desc: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcUniqueidmemtraits {
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcUniqueidsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcUseridmemtraits {
    #[serde(rename = "@edc:addrinc")]
    pub edc_addrinc: String,
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcUseridsector {
    #[serde(rename = "@atdf:pagesize")]
    pub atdf_pagesize: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:erasepagesize")]
    pub edc_erasepagesize: String,
    #[serde(rename = "@edc:isotp")]
    pub edc_isotp: String,
    #[serde(rename = "@edc:issection")]
    pub edc_issection: String,
    #[serde(rename = "@edc:magicoffset")]
    pub edc_magicoffset: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
    #[serde(rename = "@edc:sectiondesc")]
    pub edc_sectiondesc: String,
    #[serde(rename = "@edc:sectionname")]
    pub edc_sectionname: String,
    #[serde(rename = "edc:DCRDef", default)]
    pub edc__d_c_r_def: Vec<EdcDcrdef>,
    #[serde(rename = "edc:SFRDef", default)]
    pub edc__s_f_r_def: Vec<EdcSfrdef>,
}

#[derive(Debug, Deserialize)]
pub struct EdcUserotpmemtraits {
    #[serde(rename = "@edc:addrinc")]
    pub edc_addrinc: String,
    #[serde(rename = "@edc:locsize")]
    pub edc_locsize: String,
    #[serde(rename = "@edc:wordimpl")]
    pub edc_wordimpl: String,
    #[serde(rename = "@edc:wordinit")]
    pub edc_wordinit: String,
    #[serde(rename = "@edc:wordsafe")]
    pub edc_wordsafe: String,
    #[serde(rename = "@edc:wordsize")]
    pub edc_wordsize: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcUserotpsector {
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcUsersignaturesspace {
    #[serde(rename = "edc:UserIDSector")]
    pub edc__user_i_d_sector: EdcUseridsector,
}

#[derive(Debug, Deserialize)]
pub struct EdcUserzone {
    #[serde(rename = "@atdf:pagesize")]
    pub atdf_pagesize: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:erasepagesize")]
    pub edc_erasepagesize: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@edc:rw")]
    pub edc_rw: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcVdd {
    #[serde(rename = "@edc:maxdefaultvoltage")]
    pub edc_maxdefaultvoltage: String,
    #[serde(rename = "@edc:maxvoltage")]
    pub edc_maxvoltage: String,
    #[serde(rename = "@edc:maxvoltageissuggestion")]
    pub edc_maxvoltageissuggestion: String,
    #[serde(rename = "@edc:mindefaultvoltage")]
    pub edc_mindefaultvoltage: String,
    #[serde(rename = "@edc:minvoltage")]
    pub edc_minvoltage: String,
    #[serde(rename = "@edc:nominalvoltage")]
    pub edc_nominalvoltage: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcVpp {
    #[serde(rename = "@edc:defaultvoltage")]
    pub edc_defaultvoltage: String,
    #[serde(rename = "@edc:maxvoltage")]
    pub edc_maxvoltage: String,
    #[serde(rename = "@edc:minvoltage")]
    pub edc_minvoltage: String,
    #[serde(rename = "@edc:nominalvoltage")]
    pub edc_nominalvoltage: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcVectorarea {
    #[serde(rename = "@edc:nzsize")]
    pub edc_nzsize: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcVectorsector {
    #[serde(rename = "@beginaddr")]
    pub beginaddr: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@endaddr")]
    pub endaddr: String,
    #[serde(rename = "@regionid")]
    pub regionid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcVirtualpin {
    #[serde(rename = "@edc:name")]
    pub edc_name: String,
    #[serde(rename = "@edc:ppsfunction")]
    pub edc_ppsfunction: String,
    #[serde(rename = "@edc:ppsgroup")]
    pub edc_ppsgroup: String,
    #[serde(rename = "@edc:ppsval")]
    pub edc_ppsval: String,
    #[serde(rename = "@ltx:memberofperipheral")]
    pub ltx_memberofperipheral: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcWormhole {
    #[serde(rename = "@dstaddr")]
    pub dstaddr: String,
    #[serde(rename = "@dstmask")]
    pub dstmask: String,
    #[serde(rename = "@edc:dstaddr")]
    pub edc_dstaddr: String,
    #[serde(rename = "@edc:dstmask")]
    pub edc_dstmask: String,
    #[serde(rename = "@edc:srcaddr")]
    pub edc_srcaddr: String,
    #[serde(rename = "@edc:srcmask")]
    pub edc_srcmask: String,
    #[serde(rename = "@srcaddr")]
    pub srcaddr: String,
    #[serde(rename = "@srcmask")]
    pub srcmask: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcWormholesector {
    #[serde(rename = "@beginaddr")]
    pub beginaddr: String,
    #[serde(rename = "@edc:beginaddr")]
    pub edc_beginaddr: String,
    #[serde(rename = "@edc:endaddr")]
    pub edc_endaddr: String,
    #[serde(rename = "@edc:regionid")]
    pub edc_regionid: String,
    #[serde(rename = "@endaddr")]
    pub endaddr: String,
    #[serde(rename = "@regionid")]
    pub regionid: String,
    #[serde(rename = "edc:AdjustPoint", default)]
    pub edc__adjust_point: Vec<EdcAdjustpoint>,
    #[serde(rename = "edc:DCRDef", default)]
    pub edc__d_c_r_def: Vec<EdcDcrdef>,
    #[serde(rename = "edc:WORMHole", default)]
    pub edc__w_o_r_m_hole: Vec<EdcWormhole>,
}

#[derive(Debug, Deserialize)]
pub struct EdcWait {
    #[serde(rename = "@edc:cfg")]
    pub edc_cfg: String,
    #[serde(rename = "@edc:eedata")]
    pub edc_eedata: String,
    #[serde(rename = "@edc:erase")]
    pub edc_erase: String,
    #[serde(rename = "@edc:flashdata")]
    pub edc_flashdata: String,
    #[serde(rename = "@edc:lverase")]
    pub edc_lverase: String,
    #[serde(rename = "@edc:lvpgm")]
    pub edc_lvpgm: String,
    #[serde(rename = "@edc:pgm")]
    pub edc_pgm: String,
    #[serde(rename = "@edc:rowerase")]
    pub edc_rowerase: String,
    #[serde(rename = "@edc:userid")]
    pub edc_userid: String,
}

#[derive(Debug, Deserialize)]
pub struct EdcWatchdogtimer {
    #[serde(rename = "@edc:min")]
    pub edc_min: String,
}

