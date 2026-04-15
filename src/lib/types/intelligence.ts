// ── Domain Info Types (web-analyzer::domain_info) ────────────────────────────

export interface WhoisInfo {
	registrar: string;
	creation_date: string;
	expiry_date: string;
	last_updated: string;
	domain_status: string[];
	registrant: string;
	privacy_protection: string;
	name_servers: string[];
}

export interface SslInfo {
	status: string;
	issued_to: string | null;
	issuer: string | null;
	protocol_version: string | null;
	expiry_date: string | null;
	days_until_expiry: number | null;
	alternative_names: string[];
}

export interface DnsInfo {
	nameservers: string[];
	mx_records: string[];
	txt_records: string[];
	spf: string | null;
	dmarc: string | null;
}

export interface SecurityInfo {
	https_available: boolean;
	https_redirect: boolean;
	security_headers: Record<string, string>;
	headers_count: number;
}

export interface DomainInfoResult {
	domain: string;
	ipv4: string | null;
	ipv6: string[];
	all_ipv4: string[];
	reverse_dns: string | null;
	whois: WhoisInfo;
	ssl: SslInfo;
	dns: DnsInfo;
	open_ports: string[];
	http_status: string | null;
	web_server: string | null;
	response_time_ms: number | null;
	security: SecurityInfo;
	security_score: number;
}

// ── Domain DNS Types (web-analyzer::domain_dns) ──────────────────────────────

export interface DnsRecords {
	a: string[];
	aaaa: string[];
	mx: string[];
	ns: string[];
	soa: string[];
	txt: string[];
	cname: string[];
}

export interface DomainDnsResult {
	timestamp: string;
	domain: string;
	records: DnsRecords;
	response_time_ms: number;
}

// ── SEO Analysis Types (web-analyzer::seo_analysis) ──────────────────────────

export interface TitleAnalysis {
	text: string;
	length: number;
	status: string;
}

export interface MetaDescAnalysis {
	text: string;
	length: number;
	status: string;
}

export interface BasicSeoResult {
	title: TitleAnalysis;
	meta_description: MetaDescAnalysis;
	meta_keywords: string;
	canonical_url: string;
	meta_robots: string;
	viewport: string;
	language: string;
	charset: string;
}

export interface HeadingInfo {
	count: number;
	texts: string[];
}

export interface KeywordInfo {
	word: string;
	count: number;
	density: string;
}

export interface ContentAnalysisResult {
	headings: Record<string, HeadingInfo>;
	heading_issues: string[];
	word_count: number;
	word_count_status: string;
	paragraphs: number;
	text_to_html_ratio: string;
	top_keywords: KeywordInfo[];
}

export interface TechnicalSeoResult {
	page_size_bytes: number;
	http_status: number;
	redirects: number;
	internal_links: number;
	external_links: number;
	structured_data_count: number;
	has_breadcrumbs: boolean;
}

export interface SocialMediaResult {
	open_graph: Record<string, string>;
	twitter_cards: Record<string, string>;
}

export interface PerformanceResult {
	load_time_secs: number;
	load_time_status: string;
	content_size_kb: number;
	compression: string;
	server: string;
	cache_control: string;
	etag: boolean;
}

export interface AltAttributeResult {
	total_images: number;
	images_with_alt: number;
	missing_alt: number;
	alt_coverage: string;
}

export interface MobileAccessibilityResult {
	viewport_present: boolean;
	mobile_friendly: boolean;
	alt_attributes: AltAttributeResult;
	aria_labels: number;
}

export interface SchemaMarkupResult {
	json_ld_count: number;
	json_ld_types: string[];
	microdata_items: number;
	total_structured_data: number;
}

export interface LinkAnalysisResult {
	total_links: number;
	internal_links: number;
	external_links: number;
	nofollow_links: number;
}

export interface ImageSeoResult {
	total_images: number;
	lazy_loaded: number;
	with_alt_text: number;
	with_title: number;
	optimization_score: string;
}

export interface PageSpeedResult {
	css_files: number;
	js_files: number;
	inline_styles: number;
	inline_scripts: number;
	compression: string;
}

export interface SeoScoreResult {
	score: number;
	max_score: number;
	percentage: string;
	grade: string;
}

export interface SeoAnalysisResult {
	domain: string;
	basic_seo: BasicSeoResult;
	content_analysis: ContentAnalysisResult;
	technical_seo: TechnicalSeoResult;
	social_media: SocialMediaResult;
	analytics: Record<string, string>;
	performance: PerformanceResult;
	mobile_accessibility: MobileAccessibilityResult;
	seo_resources: Record<string, string>;
	schema_markup: SchemaMarkupResult;
	link_analysis: LinkAnalysisResult;
	image_seo: ImageSeoResult;
	page_speed_factors: PageSpeedResult;
	seo_score: SeoScoreResult;
}

// ── Progress & Console Types ─────────────────────────────────────────────────

export interface ScanProgressEvent {
	module: string;
	percentage: number;
	message: string;
	status: string;
}
