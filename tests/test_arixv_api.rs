use anyhow::Result;
use arxiv::ArxivQueryBuilder;

#[tokio::test]
async fn test_arxiv_api() -> Result<()> {
    let query = ArxivQueryBuilder::new()
        .search_query("machine learning")
        .start(0)
        .max_results(50)
        .sort_by("submittedDate")
        .sort_order("descending")
        .build();
    let arxivs = arxiv::fetch_arxivs(query).await?;
    for arxiv in arxivs {
        println!("{:?}, {:?}", arxiv.title, arxiv.updated);
    }
    Ok(())
}
