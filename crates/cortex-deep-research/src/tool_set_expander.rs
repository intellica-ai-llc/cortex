use serde::{Deserialize, Serialize};

/// Tool Set Expander — broader search functionality.
///
/// OpenSeeker‑v2 modification #2: "Expanding the tool set size
/// for broader functionality." A larger tool set enables the
/// agent to access more diverse information sources (web search,
/// database queries, API calls, document retrieval), producing
/// richer training trajectories.
///
/// Cortex's tool set is drawn from the Integration Fabric (30+
/// enterprise connectors) plus web search (Serper), document
/// retrieval (Knowledge Snap), and internal database query tools.
pub struct ToolSetExpander {
    available_tools: Vec<ResearchTool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTool {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tool_category: ResearchToolCategory,
    pub is_search: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResearchToolCategory {
    WebSearch,          // Serper, Google
    WebBrowsing,        // fetch and parse URLs
    InternalSearch,     // enterprise knowledge base
    DatabaseQuery,      // SQL against internal DBs
    DocumentRetrieval,  // Knowledge Snap
    Calculation,        // Python REPL
    Citation,           // reference formatting
}

impl ToolSetExpander {
    pub fn new() -> Self {
        let tools = vec![
            ResearchTool { id: "serper".into(), name: "Serper Web Search".into(),
                description: "Search the web".into(), tool_category: ResearchToolCategory::WebSearch, is_search: true },
            ResearchTool { id: "fetch_url".into(), name: "Fetch URL".into(),
                description: "Retrieve and parse a web page".into(), tool_category: ResearchToolCategory::WebBrowsing, is_search: false },
            ResearchTool { id: "internal_search".into(), name: "Enterprise Search".into(),
                description: "Search internal knowledge base".into(), tool_category: ResearchToolCategory::InternalSearch, is_search: true },
            ResearchTool { id: "sql_query".into(), name: "SQL Query".into(),
                description: "Execute SQL against internal DBs".into(), tool_category: ResearchToolCategory::DatabaseQuery, is_search: false },
            ResearchTool { id: "doc_retrieve".into(), name: "Document Retrieval".into(),
                description: "Retrieve from Knowledge Snap".into(), tool_category: ResearchToolCategory::DocumentRetrieval, is_search: false },
        ];
        Self { available_tools: tools }
    }

    /// Register additional domain‑specific tools.
    pub fn register_tool(&mut self, tool: ResearchTool) {
        self.available_tools.push(tool);
    }

    /// Get the current tool count.
    pub fn tool_count(&self) -> usize { self.available_tools.len() }

    /// List all search‑capable tools.
    pub fn search_tools(&self) -> Vec<&ResearchTool> {
        self.available_tools.iter().filter(|t| t.is_search).collect()
    }

    /// List all tools by category.
    pub fn tools_by_category(&self, category: &ResearchToolCategory) -> Vec<&ResearchTool> {
        self.available_tools.iter().filter(|t| &t.tool_category == category).collect()
    }
}
