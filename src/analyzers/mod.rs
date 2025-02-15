pub mod simple;

pub trait TrendingAnalyzer {
    type Raw;
    type Ctx;

    fn top_k<F: Fn(&Self::Ctx) -> Vec<String>>(
        &self,
        data: &Vec<(Self::Raw, Self::Ctx)>,
        f: F,
    ) -> Vec<(String, Vec<Self::Raw>)>;
}
