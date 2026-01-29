#!/usr/bin/env python3
"""
نظام تحويل النصوص الإسلامية إلى Vectors
Islamic Text to Vector Conversion Pipeline

بناءً على نتائج التقييم، يستخدم هذا النظام نموذج paraphrase-multilingual-MiniLM-L12-v2
كنموذج أساسي لتحويل النصوص العربية والإسلامية إلى vectors للبحث الدلالي.
"""

import os
import json
import time
import hashlib
import logging
from typing import List, Dict, Optional, Tuple, Any
from dataclasses import dataclass, asdict
from pathlib import Path
import numpy as np
from sentence_transformers import SentenceTransformer
from qdrant_client import QdrantClient
from qdrant_client.models import Distance, VectorParams, PointStruct
import arabic_reshaper
from bidi.algorithm import get_display

# إعداد التسجيل
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

@dataclass
class IslamicDocument:
    """وثيقة إسلامية للفهرسة"""
    id: str
    text: str
    content_type: str  # 'quran', 'hadith', 'tafsir', 'story', 'fiqh', 'akhlaq'
    source: str
    metadata: Dict[str, Any]
    embedding: Optional[List[float]] = None
    
    def to_dict(self) -> Dict[str, Any]:
        """تحويل إلى قاموس للتخزين"""
        return asdict(self)

@dataclass
class SearchResult:
    """نتيجة البحث الدلالي"""
    document: IslamicDocument
    similarity_score: float
    rank: int

class ArabicTextProcessor:
    """معالج النصوص العربية"""
    
    @staticmethod
    def normalize_arabic_text(text: str) -> str:
        """تطبيع النص العربي"""
        # إزالة التشكيل الزائد
        arabic_diacritics = "ًٌٍَُِّْ"
        for diacritic in arabic_diacritics:
            text = text.replace(diacritic, "")
        
        # توحيد الألف
        text = text.replace("أ", "ا").replace("إ", "ا").replace("آ", "ا")
        
        # توحيد التاء المربوطة والهاء
        text = text.replace("ة", "ه")
        
        # إزالة المسافات الزائدة
        text = " ".join(text.split())
        
        return text.strip()
    
    @staticmethod
    def prepare_for_display(text: str) -> str:
        """تحضير النص للعرض مع دعم RTL"""
        reshaped_text = arabic_reshaper.reshape(text)
        return get_display(reshaped_text)
    
    @staticmethod
    def extract_keywords(text: str) -> List[str]:
        """استخراج الكلمات المفتاحية من النص العربي"""
        # قائمة الكلمات الشائعة التي يجب تجاهلها
        stop_words = {
            "في", "من", "إلى", "على", "عن", "مع", "هذا", "هذه", "ذلك", "تلك",
            "التي", "الذي", "التي", "اللذان", "اللتان", "الذين", "اللواتي",
            "هو", "هي", "هم", "هن", "أن", "إن", "كان", "كانت", "يكون", "تكون"
        }
        
        words = text.split()
        keywords = [word for word in words if len(word) > 2 and word not in stop_words]
        return keywords[:10]  # أول 10 كلمات مفتاحية

class IslamicEmbeddingPipeline:
    """نظام تحويل النصوص الإسلامية إلى Embeddings"""
    
    def __init__(self, 
                 model_name: str = "sentence-transformers/paraphrase-multilingual-MiniLM-L12-v2",
                 qdrant_host: str = "localhost",
                 qdrant_port: int = 6333,
                 collection_name: str = "islamic_content"):
        """
        تهيئة النظام
        
        Args:
            model_name: اسم نموذج الـ Embedding
            qdrant_host: عنوان خادم Qdrant
            qdrant_port: منفذ خادم Qdrant
            collection_name: اسم المجموعة في قاعدة البيانات
        """
        self.model_name = model_name
        self.collection_name = collection_name
        self.text_processor = ArabicTextProcessor()
        
        # تحميل نموذج الـ Embedding
        logger.info(f"تحميل نموذج الـ Embedding: {model_name}")
        self.model = SentenceTransformer(model_name)
        self.embedding_dim = self.model.get_sentence_embedding_dimension()
        logger.info(f"تم تحميل النموذج بنجاح. أبعاد الـ Embedding: {self.embedding_dim}")
        
        # الاتصال بـ Qdrant
        try:
            self.qdrant_client = QdrantClient(host=qdrant_host, port=qdrant_port)
            logger.info(f"تم الاتصال بـ Qdrant على {qdrant_host}:{qdrant_port}")
            self._setup_collection()
        except Exception as e:
            logger.warning(f"فشل الاتصال بـ Qdrant: {e}. سيتم العمل بدون قاعدة بيانات vector")
            self.qdrant_client = None
    
    def _setup_collection(self):
        """إعداد مجموعة البيانات في Qdrant"""
        if not self.qdrant_client:
            return
            
        try:
            # التحقق من وجود المجموعة
            collections = self.qdrant_client.get_collections()
            collection_exists = any(col.name == self.collection_name for col in collections.collections)
            
            if not collection_exists:
                # إنشاء مجموعة جديدة
                self.qdrant_client.create_collection(
                    collection_name=self.collection_name,
                    vectors_config=VectorParams(
                        size=self.embedding_dim,
                        distance=Distance.COSINE
                    )
                )
                logger.info(f"تم إنشاء مجموعة جديدة: {self.collection_name}")
            else:
                logger.info(f"المجموعة موجودة بالفعل: {self.collection_name}")
                
        except Exception as e:
            logger.error(f"خطأ في إعداد مجموعة Qdrant: {e}")
    
    def generate_embedding(self, text: str) -> List[float]:
        """تحويل النص إلى embedding"""
        try:
            # تطبيع النص
            normalized_text = self.text_processor.normalize_arabic_text(text)
            
            # توليد الـ embedding
            embedding = self.model.encode(normalized_text)
            
            # تحويل إلى قائمة Python عادية
            return embedding.tolist()
            
        except Exception as e:
            logger.error(f"خطأ في توليد الـ embedding للنص: {text[:50]}... - {e}")
            return []
    
    def index_document(self, document: IslamicDocument) -> bool:
        """فهرسة وثيقة إسلامية"""
        try:
            # توليد الـ embedding إذا لم يكن موجوداً
            if not document.embedding:
                document.embedding = self.generate_embedding(document.text)
            
            if not document.embedding:
                logger.error(f"فشل في توليد embedding للوثيقة: {document.id}")
                return False
            
            # إضافة معلومات إضافية للـ metadata
            document.metadata.update({
                "keywords": self.text_processor.extract_keywords(document.text),
                "text_length": len(document.text),
                "indexed_at": time.time()
            })
            
            # حفظ في Qdrant إذا كان متاحاً
            if self.qdrant_client:
                point = PointStruct(
                    id=self._generate_point_id(document.id),
                    vector=document.embedding,
                    payload={
                        "id": document.id,
                        "text": document.text,
                        "content_type": document.content_type,
                        "source": document.source,
                        "metadata": document.metadata
                    }
                )
                
                self.qdrant_client.upsert(
                    collection_name=self.collection_name,
                    points=[point]
                )
                
                logger.info(f"تم فهرسة الوثيقة: {document.id}")
            
            return True
            
        except Exception as e:
            logger.error(f"خطأ في فهرسة الوثيقة {document.id}: {e}")
            return False
    
    def index_documents_batch(self, documents: List[IslamicDocument], batch_size: int = 100) -> int:
        """فهرسة مجموعة من الوثائق دفعة واحدة"""
        indexed_count = 0
        
        for i in range(0, len(documents), batch_size):
            batch = documents[i:i + batch_size]
            
            # توليد embeddings للدفعة
            texts = [doc.text for doc in batch]
            embeddings = self.model.encode(texts)
            
            # تحديث الوثائق بالـ embeddings
            for doc, embedding in zip(batch, embeddings):
                doc.embedding = embedding.tolist()
            
            # فهرسة الدفعة
            if self.qdrant_client:
                points = []
                for doc in batch:
                    doc.metadata.update({
                        "keywords": self.text_processor.extract_keywords(doc.text),
                        "text_length": len(doc.text),
                        "indexed_at": time.time()
                    })
                    
                    points.append(PointStruct(
                        id=self._generate_point_id(doc.id),
                        vector=doc.embedding,
                        payload={
                            "id": doc.id,
                            "text": doc.text,
                            "content_type": doc.content_type,
                            "source": doc.source,
                            "metadata": doc.metadata
                        }
                    ))
                
                try:
                    self.qdrant_client.upsert(
                        collection_name=self.collection_name,
                        points=points
                    )
                    indexed_count += len(batch)
                    logger.info(f"تم فهرسة {len(batch)} وثيقة. المجموع: {indexed_count}")
                    
                except Exception as e:
                    logger.error(f"خطأ في فهرسة الدفعة: {e}")
        
        return indexed_count
    
    def semantic_search(self, 
                       query: str, 
                       limit: int = 10,
                       content_types: Optional[List[str]] = None,
                       min_similarity: float = 0.5) -> List[SearchResult]:
        """البحث الدلالي في المحتوى الإسلامي"""
        
        if not self.qdrant_client:
            logger.error("Qdrant غير متاح للبحث")
            return []
        
        try:
            # توليد embedding للاستعلام
            query_embedding = self.generate_embedding(query)
            if not query_embedding:
                return []
            
            # إعداد فلاتر البحث
            search_filter = None
            if content_types:
                search_filter = {
                    "must": [
                        {
                            "key": "content_type",
                            "match": {"any": content_types}
                        }
                    ]
                }
            
            # تنفيذ البحث
            search_results = self.qdrant_client.search(
                collection_name=self.collection_name,
                query_vector=query_embedding,
                query_filter=search_filter,
                limit=limit,
                score_threshold=min_similarity
            )
            
            # تحويل النتائج
            results = []
            for i, result in enumerate(search_results):
                doc = IslamicDocument(
                    id=result.payload["id"],
                    text=result.payload["text"],
                    content_type=result.payload["content_type"],
                    source=result.payload["source"],
                    metadata=result.payload["metadata"],
                    embedding=None  # لا نحتاج لإرجاع الـ embedding
                )
                
                results.append(SearchResult(
                    document=doc,
                    similarity_score=result.score,
                    rank=i + 1
                ))
            
            logger.info(f"تم العثور على {len(results)} نتيجة للاستعلام: {query[:50]}...")
            return results
            
        except Exception as e:
            logger.error(f"خطأ في البحث الدلالي: {e}")
            return []
    
    def get_similar_documents(self, document_id: str, limit: int = 5) -> List[SearchResult]:
        """العثور على وثائق مشابهة لوثيقة معينة"""
        if not self.qdrant_client:
            return []
        
        try:
            # البحث عن الوثيقة الأصلية
            search_results = self.qdrant_client.scroll(
                collection_name=self.collection_name,
                scroll_filter={
                    "must": [
                        {
                            "key": "id",
                            "match": {"value": document_id}
                        }
                    ]
                },
                limit=1
            )
            
            if not search_results[0]:
                logger.warning(f"لم يتم العثور على الوثيقة: {document_id}")
                return []
            
            # استخدام embedding الوثيقة الأصلية للبحث
            original_doc = search_results[0][0]
            similar_results = self.qdrant_client.search(
                collection_name=self.collection_name,
                query_vector=original_doc.vector,
                limit=limit + 1  # +1 لاستبعاد الوثيقة الأصلية
            )
            
            # تحويل النتائج واستبعاد الوثيقة الأصلية
            results = []
            for i, result in enumerate(similar_results):
                if result.payload["id"] != document_id:  # استبعاد الوثيقة الأصلية
                    doc = IslamicDocument(
                        id=result.payload["id"],
                        text=result.payload["text"],
                        content_type=result.payload["content_type"],
                        source=result.payload["source"],
                        metadata=result.payload["metadata"]
                    )
                    
                    results.append(SearchResult(
                        document=doc,
                        similarity_score=result.score,
                        rank=len(results) + 1
                    ))
                    
                    if len(results) >= limit:
                        break
            
            return results
            
        except Exception as e:
            logger.error(f"خطأ في البحث عن وثائق مشابهة: {e}")
            return []
    
    def _generate_point_id(self, document_id: str) -> int:
        """توليد ID رقمي للنقطة في Qdrant"""
        return int(hashlib.md5(document_id.encode()).hexdigest()[:8], 16)
    
    def get_collection_stats(self) -> Dict[str, Any]:
        """إحصائيات المجموعة"""
        if not self.qdrant_client:
            return {}
        
        try:
            info = self.qdrant_client.get_collection(self.collection_name)
            return {
                "total_documents": info.points_count,
                "vector_size": info.config.params.vectors.size,
                "distance_metric": info.config.params.vectors.distance.value
            }
        except Exception as e:
            logger.error(f"خطأ في الحصول على إحصائيات المجموعة: {e}")
            return {}
    
    def export_embeddings(self, output_file: str) -> bool:
        """تصدير الـ embeddings إلى ملف"""
        if not self.qdrant_client:
            return False
        
        try:
            # استخراج جميع النقاط
            all_points = []
            offset = None
            
            while True:
                result = self.qdrant_client.scroll(
                    collection_name=self.collection_name,
                    limit=1000,
                    offset=offset,
                    with_vectors=True
                )
                
                points, next_offset = result
                if not points:
                    break
                
                for point in points:
                    all_points.append({
                        "id": point.payload["id"],
                        "text": point.payload["text"],
                        "content_type": point.payload["content_type"],
                        "source": point.payload["source"],
                        "embedding": point.vector,
                        "metadata": point.payload["metadata"]
                    })
                
                offset = next_offset
                if not next_offset:
                    break
            
            # حفظ في ملف JSON
            with open(output_file, 'w', encoding='utf-8') as f:
                json.dump(all_points, f, ensure_ascii=False, indent=2)
            
            logger.info(f"تم تصدير {len(all_points)} embedding إلى {output_file}")
            return True
            
        except Exception as e:
            logger.error(f"خطأ في تصدير الـ embeddings: {e}")
            return False

def create_sample_islamic_documents() -> List[IslamicDocument]:
    """إنشاء مجموعة عينة من الوثائق الإسلامية للاختبار"""
    return [
        IslamicDocument(
            id="quran_001",
            text="بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
            content_type="quran",
            source="الفاتحة:1",
            metadata={"surah": "الفاتحة", "ayah": 1}
        ),
        IslamicDocument(
            id="quran_002",
            text="الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ",
            content_type="quran",
            source="الفاتحة:2",
            metadata={"surah": "الفاتحة", "ayah": 2}
        ),
        IslamicDocument(
            id="hadith_001",
            text="إنما الأعمال بالنيات وإنما لكل امرئ ما نوى",
            content_type="hadith",
            source="صحيح البخاري",
            metadata={"book": "البخاري", "grade": "صحيح"}
        ),
        IslamicDocument(
            id="hadith_002",
            text="من كان يؤمن بالله واليوم الآخر فليقل خيراً أو ليصمت",
            content_type="hadith",
            source="صحيح البخاري",
            metadata={"book": "البخاري", "grade": "صحيح"}
        ),
        IslamicDocument(
            id="tafsir_001",
            text="الحمد لله رب العالمين: أي الثناء على الله بصفاته التي كلها أوصاف كمال",
            content_type="tafsir",
            source="تفسير ابن كثير",
            metadata={"mufassir": "ابن كثير", "surah": "الفاتحة"}
        )
    ]

def main():
    """مثال على استخدام النظام"""
    logger.info("بدء تشغيل نظام تحويل النصوص الإسلامية إلى Vectors")
    
    # إنشاء النظام
    pipeline = IslamicEmbeddingPipeline()
    
    # إنشاء وثائق عينة
    sample_docs = create_sample_islamic_documents()
    
    # فهرسة الوثائق
    indexed_count = pipeline.index_documents_batch(sample_docs)
    logger.info(f"تم فهرسة {indexed_count} وثيقة")
    
    # اختبار البحث الدلالي
    search_queries = [
        "البسملة وبداية السور",
        "الحمد والثناء على الله",
        "أهمية النية في الأعمال",
        "آداب الكلام والصمت"
    ]
    
    for query in search_queries:
        logger.info(f"\nالبحث عن: {query}")
        results = pipeline.semantic_search(query, limit=3)
        
        for result in results:
            logger.info(f"  - [{result.rank}] {result.document.content_type}: {result.document.text[:50]}... "
                       f"(التشابه: {result.similarity_score:.3f})")
    
    # إحصائيات المجموعة
    stats = pipeline.get_collection_stats()
    if stats:
        logger.info(f"\nإحصائيات المجموعة: {stats}")

if __name__ == "__main__":
    main()