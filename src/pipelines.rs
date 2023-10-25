use mongodb::bson::{doc, Document};

pub fn create_search_pipe(q: &String, boost: bool, limit: i64) -> Vec<Document> {
    let search_pipeline = vec![
        doc! {
            "$search": {
                "index": "default",
                "text": {
                    "query": q,
                    "path": ["text_content", "title", "url"],
                    "fuzzy": {}, // use default fuzzy options
                },
                "scoreDetails": true,
            },
        },
        doc! {
            "$limit": limit,
        },
        doc! {
            "$addFields": {
                "score": { "$meta": "searchScoreDetails" },
            },
        },
    ];

    let boost_pipeline = vec![
        doc! {
            "$search": {
                "index": "default",
                "text": {
                    "query": q,
                    "path": ["text_content", "title", "url"],
                    "fuzzy": {}, // use default fuzzy options
                    "score": {
                        "function": {
                            "multiply":[
                                {
                                    "add":[
                                        { "path": "page_rank" },
                                        { "constant": 1 },
                                    ]
                                },
                                { "score": "relevance" },
                            ],
                        },
                    },
                },
                "scoreDetails": true,
            },
        },
        doc! {
            "$limit": limit,
        },
        doc! {
            "$addFields": {
                "score": { "$meta": "searchScoreDetails" },
            },
        },
    ];

    // let boost_pipeline = vec![
    //     doc! {
    //         "$search": {
    //             "index": "default",
    //             "text": {
    //                 "query": q,
    //                 "path": ["text_content", "title", "url"],
    //                 "fuzzy": {}, // use default fuzzy options
    //                 "score": {
    //                     "boost": {
    //                         "path": "page_rank",
    //                     },
    //                 },
    //             },
    //             "scoreDetails": true,
    //         },
    //     },
    //     doc! {
    //         "$limit": limit,
    //     },
    //     doc! {
    //         "$addFields": {
    //             "score": { "$meta": "searchScoreDetails" },
    //         },
    //     },
    // ];

    if boost {
        boost_pipeline
    } else {
        search_pipeline
    }
}

pub fn create_random_pipe(limit: i64) -> Vec<Document> { 
    let random_pipeline = vec![
        doc! {
            "$sample": {
                "size": limit,
            },
        },
        doc! {
            "$addFields": {
                "score": { "$meta": "searchScoreDetails" },
            },
        },
    ];
    random_pipeline  
}
