# تقرير تقييم نماذج الـ Embedding العربية

## النتائج المقارنة

| النموذج | Precision@5 | Recall@5 | MRR | وقت المعالجة (ثانية) | الذاكرة (MB) | الدقة الدلالية |
|---------|-------------|----------|-----|-------------------|------------|---------------|
| paraphrase-multilingual-MiniLM | 0.200 | 0.875 | 0.653 | 0.0098 | 448.8 | 0.736 |
| paraphrase-multilingual-mpnet- | 0.175 | 0.750 | 0.726 | 0.0269 | 1060.7 | 0.598 |

## التحليل والتوصيات

- **أفضل دقة**: sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2 (Precision@5: 0.200)
- **أسرع معالجة**: sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2 (0.0098 ثانية)
- **أقل استهلاك ذاكرة**: sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2 (448.8 MB)
