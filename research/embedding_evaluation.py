#!/usr/bin/env python3
"""
تقييم نماذج الـ Embedding العربية للتطبيق الإسلامي الشامل
Arabic Embedding Models Evaluation for Islamic Comprehensive App
"""

import os
import json
import time
import numpy as np
from typing import List, Dict, Tuple, Any
from dataclasses import dataclass
from sentence_transformers import SentenceTransformer
from sklearn.metrics.pairwise import cosine_similarity
import logging

# إعداد التسجيل
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

@dataclass
class EvaluationResult:
    """نتيجة تقييم نموذج"""
    model_name: str
    precision_at_5: float
    recall_at_5: float
    mrr: float
    avg_processing_time: float
    memory_usage_mb: float
    semantic_accuracy: float

class IslamicTextEvaluator:
    """مقيم النصوص الإسلامية لنماذج الـ Embedding"""
    
    def __init__(self):
        self.test_corpus = self._load_test_corpus()
        self.test_queries = self._load_test_queries()
        self.ground_truth = self._load_ground_truth()
        
    def _load_test_corpus(self) -> List[Dict[str, Any]]:
        """تحميل مجموعة النصوص التجريبية"""
        return [
            {
                "id": "quran_001",
                "text": "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
                "type": "quran",
                "source": "الفاتحة:1"
            },
            {
                "id": "quran_002", 
                "text": "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ",
                "type": "quran",
                "source": "الفاتحة:2"
            },
            {
                "id": "quran_003",
                "text": "وَمَا خَلَقْتُ الْجِنَّ وَالْإِنسَ إِلَّا لِيَعْبُدُونِ",
                "type": "quran", 
                "source": "الذاريات:56"
            },
            {
                "id": "hadith_001",
                "text": "إنما الأعمال بالنيات وإنما لكل امرئ ما نوى",
                "type": "hadith",
                "source": "البخاري"
            },
            {
                "id": "hadith_002",
                "text": "من كان يؤمن بالله واليوم الآخر فليقل خيراً أو ليصمت",
                "type": "hadith",
                "source": "البخاري"
            },
            {
                "id": "hadith_003",
                "text": "المسلم من سلم المسلمون من لسانه ويده",
                "type": "hadith",
                "source": "البخاري"
            },
            {
                "id": "tafsir_001",
                "text": "الحمد لله رب العالمين: أي الثناء على الله بصفاته التي كلها أوصاف كمال",
                "type": "tafsir",
                "source": "تفسير ابن كثير"
            },
            {
                "id": "story_001",
                "text": "كان النبي صلى الله عليه وسلم أصدق الناس وأكرمهم وأشجعهم",
                "type": "story",
                "source": "السيرة النبوية"
            },
            {
                "id": "fiqh_001",
                "text": "الصلاة عماد الدين وهي أول ما يحاسب عليه العبد يوم القيامة",
                "type": "fiqh",
                "source": "الفقه الإسلامي"
            },
            {
                "id": "akhlaq_001",
                "text": "الصبر نصف الإيمان والشكر نصف الإيمان والطهارة الإيمان كله",
                "type": "akhlaq",
                "source": "الأخلاق الإسلامية"
            }
        ]
    
    def _load_test_queries(self) -> List[Dict[str, Any]]:
        """تحميل استعلامات الاختبار"""
        return [
            {
                "id": "q001",
                "text": "البسملة وبداية السور",
                "intent": "البحث عن البسملة",
                "expected_types": ["quran"]
            },
            {
                "id": "q002", 
                "text": "الحمد والثناء على الله",
                "intent": "البحث عن آيات الحمد",
                "expected_types": ["quran", "tafsir"]
            },
            {
                "id": "q003",
                "text": "الغرض من خلق الإنسان",
                "intent": "البحث عن حكمة الخلق",
                "expected_types": ["quran"]
            },
            {
                "id": "q004",
                "text": "أهمية النية في الأعمال",
                "intent": "البحث عن أحاديث النية",
                "expected_types": ["hadith"]
            },
            {
                "id": "q005",
                "text": "آداب الكلام والصمت",
                "intent": "البحث عن آداب الحديث",
                "expected_types": ["hadith"]
            },
            {
                "id": "q006",
                "text": "صفات المسلم الحق",
                "intent": "البحث عن صفات المؤمن",
                "expected_types": ["hadith", "akhlaq"]
            },
            {
                "id": "q007",
                "text": "أهمية الصلاة في الإسلام",
                "intent": "البحث عن فضل الصلاة",
                "expected_types": ["fiqh", "hadith"]
            },
            {
                "id": "q008",
                "text": "الصبر والشكر في الإيمان",
                "intent": "البحث عن فضائل الصبر",
                "expected_types": ["akhlaq", "hadith"]
            }
        ]
    
    def _load_ground_truth(self) -> Dict[str, List[str]]:
        """تحميل الإجابات الصحيحة المتوقعة"""
        return {
            "q001": ["quran_001"],
            "q002": ["quran_002", "tafsir_001"],
            "q003": ["quran_003"],
            "q004": ["hadith_001"],
            "q005": ["hadith_002"],
            "q006": ["hadith_003"],
            "q007": ["fiqh_001"],
            "q008": ["akhlaq_001"]
        }
    
    def evaluate_model(self, model_name: str) -> EvaluationResult:
        """تقييم نموذج معين"""
        logger.info(f"بدء تقييم النموذج: {model_name}")
        
        try:
            # تحميل النموذج
            start_time = time.time()
            model = SentenceTransformer(model_name)
            load_time = time.time() - start_time
            logger.info(f"تم تحميل النموذج في {load_time:.2f} ثانية")
            
            # تحويل النصوص إلى embeddings
            corpus_texts = [doc["text"] for doc in self.test_corpus]
            query_texts = [q["text"] for q in self.test_queries]
            
            # قياس وقت المعالجة
            start_time = time.time()
            corpus_embeddings = model.encode(corpus_texts)
            query_embeddings = model.encode(query_texts)
            processing_time = (time.time() - start_time) / len(corpus_texts + query_texts)
            
            # حساب التشابه
            similarities = cosine_similarity(query_embeddings, corpus_embeddings)
            
            # تقييم النتائج
            precision_scores = []
            recall_scores = []
            mrr_scores = []
            
            for i, query in enumerate(self.test_queries):
                query_id = query["id"]
                expected_docs = self.ground_truth.get(query_id, [])
                
                if not expected_docs:
                    continue
                
                # ترتيب النتائج حسب التشابه
                doc_scores = [(j, similarities[i][j]) for j in range(len(self.test_corpus))]
                doc_scores.sort(key=lambda x: x[1], reverse=True)
                
                # حساب Precision@5 و Recall@5
                top_5_docs = [self.test_corpus[j]["id"] for j, _ in doc_scores[:5]]
                relevant_in_top5 = len(set(top_5_docs) & set(expected_docs))
                
                precision_at_5 = relevant_in_top5 / 5
                recall_at_5 = relevant_in_top5 / len(expected_docs)
                
                precision_scores.append(precision_at_5)
                recall_scores.append(recall_at_5)
                
                # حساب MRR
                for rank, (doc_idx, _) in enumerate(doc_scores, 1):
                    if self.test_corpus[doc_idx]["id"] in expected_docs:
                        mrr_scores.append(1.0 / rank)
                        break
                else:
                    mrr_scores.append(0.0)
            
            # حساب الدقة الدلالية (تقييم نوعي مبسط)
            semantic_accuracy = self._evaluate_semantic_accuracy(
                model, query_texts, corpus_texts
            )
            
            result = EvaluationResult(
                model_name=model_name,
                precision_at_5=np.mean(precision_scores),
                recall_at_5=np.mean(recall_scores),
                mrr=np.mean(mrr_scores),
                avg_processing_time=processing_time,
                memory_usage_mb=self._estimate_memory_usage(model),
                semantic_accuracy=semantic_accuracy
            )
            
            logger.info(f"انتهى تقييم النموذج: {model_name}")
            return result
            
        except Exception as e:
            logger.error(f"خطأ في تقييم النموذج {model_name}: {str(e)}")
            return EvaluationResult(
                model_name=model_name,
                precision_at_5=0.0,
                recall_at_5=0.0,
                mrr=0.0,
                avg_processing_time=float('inf'),
                memory_usage_mb=float('inf'),
                semantic_accuracy=0.0
            )
    
    def _evaluate_semantic_accuracy(self, model, queries: List[str], corpus: List[str]) -> float:
        """تقييم الدقة الدلالية بطريقة مبسطة"""
        # هذا تقييم مبسط - في التطبيق الحقيقي نحتاج تقييم أكثر تفصيلاً
        try:
            # اختبار بعض الأمثلة المحددة
            test_pairs = [
                ("الله", "الرحمن"),  # يجب أن يكون التشابه عالي
                ("الصلاة", "العبادة"),  # يجب أن يكون التشابه متوسط إلى عالي
                ("الصبر", "الجزع"),  # يجب أن يكون التشابه منخفض (متضادان)
            ]
            
            scores = []
            for word1, word2 in test_pairs:
                emb1 = model.encode([word1])
                emb2 = model.encode([word2])
                similarity = cosine_similarity(emb1, emb2)[0][0]
                scores.append(similarity)
            
            # تقييم بسيط: المتوسط
            return np.mean(scores)
            
        except:
            return 0.5  # قيمة افتراضية
    
    def _estimate_memory_usage(self, model) -> float:
        """تقدير استهلاك الذاكرة (تقريبي)"""
        try:
            # تقدير بسيط بناءً على حجم النموذج
            param_count = sum(p.numel() for p in model.parameters())
            # تقدير: 4 bytes per parameter (float32)
            memory_mb = (param_count * 4) / (1024 * 1024)
            return memory_mb
        except:
            return 0.0
    
    def run_comprehensive_evaluation(self) -> Dict[str, EvaluationResult]:
        """تشغيل التقييم الشامل لجميع النماذج"""
        models_to_test = [
            "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2",
            "sentence-transformers/paraphrase-multilingual-mpnet-base-v2",
            # ملاحظة: النماذج العربية المتخصصة تحتاج معالجة إضافية
            # "aubmindlab/bert-base-arabertv02",
            # "CAMeL-Lab/bert-base-arabic-camelbert-mix"
        ]
        
        results = {}
        for model_name in models_to_test:
            try:
                result = self.evaluate_model(model_name)
                results[model_name] = result
            except Exception as e:
                logger.error(f"فشل في تقييم النموذج {model_name}: {str(e)}")
                continue
        
        return results
    
    def generate_report(self, results: Dict[str, EvaluationResult]) -> str:
        """إنشاء تقرير التقييم"""
        report = "# تقرير تقييم نماذج الـ Embedding العربية\n\n"
        report += "## النتائج المقارنة\n\n"
        report += "| النموذج | Precision@5 | Recall@5 | MRR | وقت المعالجة (ثانية) | الذاكرة (MB) | الدقة الدلالية |\n"
        report += "|---------|-------------|----------|-----|-------------------|------------|---------------|\n"
        
        for model_name, result in results.items():
            short_name = model_name.split('/')[-1][:30]
            report += f"| {short_name} | {result.precision_at_5:.3f} | {result.recall_at_5:.3f} | {result.mrr:.3f} | {result.avg_processing_time:.4f} | {result.memory_usage_mb:.1f} | {result.semantic_accuracy:.3f} |\n"
        
        report += "\n## التحليل والتوصيات\n\n"
        
        if results:
            best_precision = max(results.values(), key=lambda x: x.precision_at_5)
            best_speed = min(results.values(), key=lambda x: x.avg_processing_time)
            best_memory = min(results.values(), key=lambda x: x.memory_usage_mb)
            
            report += f"- **أفضل دقة**: {best_precision.model_name} (Precision@5: {best_precision.precision_at_5:.3f})\n"
            report += f"- **أسرع معالجة**: {best_speed.model_name} ({best_speed.avg_processing_time:.4f} ثانية)\n"
            report += f"- **أقل استهلاك ذاكرة**: {best_memory.model_name} ({best_memory.memory_usage_mb:.1f} MB)\n"
        
        return report

def main():
    """الدالة الرئيسية"""
    logger.info("بدء تقييم نماذج الـ Embedding العربية")
    
    evaluator = IslamicTextEvaluator()
    results = evaluator.run_comprehensive_evaluation()
    
    # حفظ النتائج
    results_dict = {
        model_name: {
            "precision_at_5": float(result.precision_at_5),
            "recall_at_5": float(result.recall_at_5),
            "mrr": float(result.mrr),
            "avg_processing_time": float(result.avg_processing_time),
            "memory_usage_mb": float(result.memory_usage_mb),
            "semantic_accuracy": float(result.semantic_accuracy)
        }
        for model_name, result in results.items()
    }
    
    with open("evaluation_results.json", "w", encoding="utf-8") as f:
        json.dump(results_dict, f, ensure_ascii=False, indent=2)
    
    # إنشاء التقرير
    report = evaluator.generate_report(results)
    with open("evaluation_report.md", "w", encoding="utf-8") as f:
        f.write(report)
    
    logger.info("انتهى التقييم. تم حفظ النتائج في evaluation_results.json")
    logger.info("تم حفظ التقرير في evaluation_report.md")

if __name__ == "__main__":
    main()