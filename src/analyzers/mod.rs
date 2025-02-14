pub mod simple;

pub trait TrendingAnalyzer {
    type Raw;
    type Ctx;

    fn problems(&self, data: Vec<(Self::Raw, Self::Ctx)>) -> Vec<(String, Vec<Self::Raw>)>;
    fn techniques(&self, data: Vec<(Self::Raw, Self::Ctx)>) -> Vec<(String, Vec<Self::Raw>)>;
    fn fields(&self, data: Vec<(Self::Raw, Self::Ctx)>) -> Vec<(String, Vec<Self::Raw>)>;
}
