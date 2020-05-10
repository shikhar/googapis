/// ################################################################ #
///
/// Represents the input to API methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// Required. If the type is not set or is `TYPE_UNSPECIFIED`,
    /// returns an `INVALID_ARGUMENT` error.
    #[prost(enumeration = "document::Type", tag = "1")]
    pub r#type: i32,
    /// The language of the document (if not specified, the language is
    /// automatically detected). Both ISO and BCP-47 language codes are
    /// accepted.<br>
    /// [Language
    /// Support](https://cloud.google.com/natural-language/docs/languages) lists
    /// currently supported languages for each API method. If the language (either
    /// specified by the caller or automatically detected) is not supported by the
    /// called API method, an `INVALID_ARGUMENT` error is returned.
    #[prost(string, tag = "4")]
    pub language: std::string::String,
    /// The source of the document: a string containing the content or a
    /// Google Cloud Storage URI.
    #[prost(oneof = "document::Source", tags = "2, 3")]
    pub source: ::std::option::Option<document::Source>,
}
pub mod document {
    /// The document types enum.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// The content type is not specified.
        Unspecified = 0,
        /// Plain text
        PlainText = 1,
        /// HTML
        Html = 2,
    }
    /// The source of the document: a string containing the content or a
    /// Google Cloud Storage URI.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The content of the input in string format.
        /// Cloud audit logging exempt since it is based on user data.
        #[prost(string, tag = "2")]
        Content(std::string::String),
        /// The Google Cloud Storage URI where the file content is located.
        /// This URI must be of the form: gs://bucket_name/object_name. For more
        /// details, see https://cloud.google.com/storage/docs/reference-uris.
        /// NOTE: Cloud Storage object versioning is not supported.
        #[prost(string, tag = "3")]
        GcsContentUri(std::string::String),
    }
}
/// Represents a sentence in the input document.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sentence {
    /// The sentence text.
    #[prost(message, optional, tag = "1")]
    pub text: ::std::option::Option<TextSpan>,
    /// For calls to [AnalyzeSentiment][] or if
    /// [AnnotateTextRequest.Features.extract_document_sentiment][google.cloud.language.v1.AnnotateTextRequest.Features.extract_document_sentiment] is set to
    /// true, this field will contain the sentiment for the sentence.
    #[prost(message, optional, tag = "2")]
    pub sentiment: ::std::option::Option<Sentiment>,
}
/// Represents a phrase in the text that is a known entity, such as
/// a person, an organization, or location. The API associates information, such
/// as salience and mentions, with entities.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// The representative name for the entity.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The entity type.
    #[prost(enumeration = "entity::Type", tag = "2")]
    pub r#type: i32,
    /// Metadata associated with the entity.
    ///
    /// For most entity types, the metadata is a Wikipedia URL (`wikipedia_url`)
    /// and Knowledge Graph MID (`mid`), if they are available. For the metadata
    /// associated with other entity types, see the Type table below.
    #[prost(map = "string, string", tag = "3")]
    pub metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The salience score associated with the entity in the [0, 1.0] range.
    ///
    /// The salience score for an entity provides information about the
    /// importance or centrality of that entity to the entire document text.
    /// Scores closer to 0 are less salient, while scores closer to 1.0 are highly
    /// salient.
    #[prost(float, tag = "4")]
    pub salience: f32,
    /// The mentions of this entity in the input document. The API currently
    /// supports proper noun mentions.
    #[prost(message, repeated, tag = "5")]
    pub mentions: ::std::vec::Vec<EntityMention>,
    /// For calls to [AnalyzeEntitySentiment][] or if
    /// [AnnotateTextRequest.Features.extract_entity_sentiment][google.cloud.language.v1.AnnotateTextRequest.Features.extract_entity_sentiment] is set to
    /// true, this field will contain the aggregate sentiment expressed for this
    /// entity in the provided document.
    #[prost(message, optional, tag = "6")]
    pub sentiment: ::std::option::Option<Sentiment>,
}
pub mod entity {
    /// The type of the entity. For most entity types, the associated metadata is a
    /// Wikipedia URL (`wikipedia_url`) and Knowledge Graph MID (`mid`). The table
    /// below lists the associated fields for entities that have different
    /// metadata.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unknown
        Unknown = 0,
        /// Person
        Person = 1,
        /// Location
        Location = 2,
        /// Organization
        Organization = 3,
        /// Event
        Event = 4,
        /// Artwork
        WorkOfArt = 5,
        /// Consumer product
        ConsumerGood = 6,
        /// Other types of entities
        Other = 7,
        /// Phone number<br><br>
        /// The metadata lists the phone number, formatted according to local
        /// convention, plus whichever additional elements appear in the text:<ul>
        /// <li><code>number</code> &ndash; the actual number, broken down into
        /// sections as per local convention</li> <li><code>national_prefix</code>
        /// &ndash; country code, if detected</li> <li><code>area_code</code> &ndash;
        /// region or area code, if detected</li> <li><code>extension</code> &ndash;
        /// phone extension (to be dialed after connection), if detected</li></ul>
        PhoneNumber = 9,
        /// Address<br><br>
        /// The metadata identifies the street number and locality plus whichever
        /// additional elements appear in the text:<ul>
        /// <li><code>street_number</code> &ndash; street number</li>
        /// <li><code>locality</code> &ndash; city or town</li>
        /// <li><code>street_name</code> &ndash; street/route name, if detected</li>
        /// <li><code>postal_code</code> &ndash; postal code, if detected</li>
        /// <li><code>country</code> &ndash; country, if detected</li>
        /// <li><code>broad_region</code> &ndash; administrative area, such as the
        /// state, if detected</li> <li><code>narrow_region</code> &ndash; smaller
        /// administrative area, such as county, if detected</li>
        /// <li><code>sublocality</code> &ndash; used in Asian addresses to demark a
        /// district within a city, if detected</li></ul>
        Address = 10,
        /// Date<br><br>
        /// The metadata identifies the components of the date:<ul>
        /// <li><code>year</code> &ndash; four digit year, if detected</li>
        /// <li><code>month</code> &ndash; two digit month number, if detected</li>
        /// <li><code>day</code> &ndash; two digit day number, if detected</li></ul>
        Date = 11,
        /// Number<br><br>
        /// The metadata is the number itself.
        Number = 12,
        /// Price<br><br>
        /// The metadata identifies the <code>value</code> and <code>currency</code>.
        Price = 13,
    }
}
/// Represents the smallest syntactic building block of the text.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    /// The token text.
    #[prost(message, optional, tag = "1")]
    pub text: ::std::option::Option<TextSpan>,
    /// Parts of speech tag for this token.
    #[prost(message, optional, tag = "2")]
    pub part_of_speech: ::std::option::Option<PartOfSpeech>,
    /// Dependency tree parse for this token.
    #[prost(message, optional, tag = "3")]
    pub dependency_edge: ::std::option::Option<DependencyEdge>,
    /// [Lemma](https://en.wikipedia.org/wiki/Lemma_%28morphology%29) of the token.
    #[prost(string, tag = "4")]
    pub lemma: std::string::String,
}
/// Represents the feeling associated with the entire text or entities in
/// the text.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sentiment {
    /// A non-negative number in the [0, +inf) range, which represents
    /// the absolute magnitude of sentiment regardless of score (positive or
    /// negative).
    #[prost(float, tag = "2")]
    pub magnitude: f32,
    /// Sentiment score between -1.0 (negative sentiment) and 1.0
    /// (positive sentiment).
    #[prost(float, tag = "3")]
    pub score: f32,
}
/// Represents part of speech information for a token. Parts of speech
/// are as defined in
/// http://www.lrec-conf.org/proceedings/lrec2012/pdf/274_Paper.pdf
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartOfSpeech {
    /// The part of speech tag.
    #[prost(enumeration = "part_of_speech::Tag", tag = "1")]
    pub tag: i32,
    /// The grammatical aspect.
    #[prost(enumeration = "part_of_speech::Aspect", tag = "2")]
    pub aspect: i32,
    /// The grammatical case.
    #[prost(enumeration = "part_of_speech::Case", tag = "3")]
    pub case: i32,
    /// The grammatical form.
    #[prost(enumeration = "part_of_speech::Form", tag = "4")]
    pub form: i32,
    /// The grammatical gender.
    #[prost(enumeration = "part_of_speech::Gender", tag = "5")]
    pub gender: i32,
    /// The grammatical mood.
    #[prost(enumeration = "part_of_speech::Mood", tag = "6")]
    pub mood: i32,
    /// The grammatical number.
    #[prost(enumeration = "part_of_speech::Number", tag = "7")]
    pub number: i32,
    /// The grammatical person.
    #[prost(enumeration = "part_of_speech::Person", tag = "8")]
    pub person: i32,
    /// The grammatical properness.
    #[prost(enumeration = "part_of_speech::Proper", tag = "9")]
    pub proper: i32,
    /// The grammatical reciprocity.
    #[prost(enumeration = "part_of_speech::Reciprocity", tag = "10")]
    pub reciprocity: i32,
    /// The grammatical tense.
    #[prost(enumeration = "part_of_speech::Tense", tag = "11")]
    pub tense: i32,
    /// The grammatical voice.
    #[prost(enumeration = "part_of_speech::Voice", tag = "12")]
    pub voice: i32,
}
pub mod part_of_speech {
    /// The part of speech tags enum.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Tag {
        /// Unknown
        Unknown = 0,
        /// Adjective
        Adj = 1,
        /// Adposition (preposition and postposition)
        Adp = 2,
        /// Adverb
        Adv = 3,
        /// Conjunction
        Conj = 4,
        /// Determiner
        Det = 5,
        /// Noun (common and proper)
        Noun = 6,
        /// Cardinal number
        Num = 7,
        /// Pronoun
        Pron = 8,
        /// Particle or other function word
        Prt = 9,
        /// Punctuation
        Punct = 10,
        /// Verb (all tenses and modes)
        Verb = 11,
        /// Other: foreign words, typos, abbreviations
        X = 12,
        /// Affix
        Affix = 13,
    }
    /// The characteristic of a verb that expresses time flow during an event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Aspect {
        /// Aspect is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Perfective
        Perfective = 1,
        /// Imperfective
        Imperfective = 2,
        /// Progressive
        Progressive = 3,
    }
    /// The grammatical function performed by a noun or pronoun in a phrase,
    /// clause, or sentence. In some languages, other parts of speech, such as
    /// adjective and determiner, take case inflection in agreement with the noun.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Case {
        /// Case is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Accusative
        Accusative = 1,
        /// Adverbial
        Adverbial = 2,
        /// Complementive
        Complementive = 3,
        /// Dative
        Dative = 4,
        /// Genitive
        Genitive = 5,
        /// Instrumental
        Instrumental = 6,
        /// Locative
        Locative = 7,
        /// Nominative
        Nominative = 8,
        /// Oblique
        Oblique = 9,
        /// Partitive
        Partitive = 10,
        /// Prepositional
        Prepositional = 11,
        /// Reflexive
        ReflexiveCase = 12,
        /// Relative
        RelativeCase = 13,
        /// Vocative
        Vocative = 14,
    }
    /// Depending on the language, Form can be categorizing different forms of
    /// verbs, adjectives, adverbs, etc. For example, categorizing inflected
    /// endings of verbs and adjectives or distinguishing between short and long
    /// forms of adjectives and participles
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Form {
        /// Form is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Adnomial
        Adnomial = 1,
        /// Auxiliary
        Auxiliary = 2,
        /// Complementizer
        Complementizer = 3,
        /// Final ending
        FinalEnding = 4,
        /// Gerund
        Gerund = 5,
        /// Realis
        Realis = 6,
        /// Irrealis
        Irrealis = 7,
        /// Short form
        Short = 8,
        /// Long form
        Long = 9,
        /// Order form
        Order = 10,
        /// Specific form
        Specific = 11,
    }
    /// Gender classes of nouns reflected in the behaviour of associated words.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Gender {
        /// Gender is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Feminine
        Feminine = 1,
        /// Masculine
        Masculine = 2,
        /// Neuter
        Neuter = 3,
    }
    /// The grammatical feature of verbs, used for showing modality and attitude.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mood {
        /// Mood is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Conditional
        ConditionalMood = 1,
        /// Imperative
        Imperative = 2,
        /// Indicative
        Indicative = 3,
        /// Interrogative
        Interrogative = 4,
        /// Jussive
        Jussive = 5,
        /// Subjunctive
        Subjunctive = 6,
    }
    /// Count distinctions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Number {
        /// Number is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Singular
        Singular = 1,
        /// Plural
        Plural = 2,
        /// Dual
        Dual = 3,
    }
    /// The distinction between the speaker, second person, third person, etc.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Person {
        /// Person is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// First
        First = 1,
        /// Second
        Second = 2,
        /// Third
        Third = 3,
        /// Reflexive
        ReflexivePerson = 4,
    }
    /// This category shows if the token is part of a proper name.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Proper {
        /// Proper is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Proper
        Proper = 1,
        /// Not proper
        NotProper = 2,
    }
    /// Reciprocal features of a pronoun.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reciprocity {
        /// Reciprocity is not applicable in the analyzed language or is not
        /// predicted.
        Unknown = 0,
        /// Reciprocal
        Reciprocal = 1,
        /// Non-reciprocal
        NonReciprocal = 2,
    }
    /// Time reference.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Tense {
        /// Tense is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Conditional
        ConditionalTense = 1,
        /// Future
        Future = 2,
        /// Past
        Past = 3,
        /// Present
        Present = 4,
        /// Imperfect
        Imperfect = 5,
        /// Pluperfect
        Pluperfect = 6,
    }
    /// The relationship between the action that a verb expresses and the
    /// participants identified by its arguments.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Voice {
        /// Voice is not applicable in the analyzed language or is not predicted.
        Unknown = 0,
        /// Active
        Active = 1,
        /// Causative
        Causative = 2,
        /// Passive
        Passive = 3,
    }
}
/// Represents dependency parse tree information for a token. (For more
/// information on dependency labels, see
/// http://www.aclweb.org/anthology/P13-2017
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DependencyEdge {
    /// Represents the head of this token in the dependency tree.
    /// This is the index of the token which has an arc going to this token.
    /// The index is the position of the token in the array of tokens returned
    /// by the API method. If this token is a root token, then the
    /// `head_token_index` is its own index.
    #[prost(int32, tag = "1")]
    pub head_token_index: i32,
    /// The parse label for the token.
    #[prost(enumeration = "dependency_edge::Label", tag = "2")]
    pub label: i32,
}
pub mod dependency_edge {
    /// The parse label enum for the token.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Label {
        /// Unknown
        Unknown = 0,
        /// Abbreviation modifier
        Abbrev = 1,
        /// Adjectival complement
        Acomp = 2,
        /// Adverbial clause modifier
        Advcl = 3,
        /// Adverbial modifier
        Advmod = 4,
        /// Adjectival modifier of an NP
        Amod = 5,
        /// Appositional modifier of an NP
        Appos = 6,
        /// Attribute dependent of a copular verb
        Attr = 7,
        /// Auxiliary (non-main) verb
        Aux = 8,
        /// Passive auxiliary
        Auxpass = 9,
        /// Coordinating conjunction
        Cc = 10,
        /// Clausal complement of a verb or adjective
        Ccomp = 11,
        /// Conjunct
        Conj = 12,
        /// Clausal subject
        Csubj = 13,
        /// Clausal passive subject
        Csubjpass = 14,
        /// Dependency (unable to determine)
        Dep = 15,
        /// Determiner
        Det = 16,
        /// Discourse
        Discourse = 17,
        /// Direct object
        Dobj = 18,
        /// Expletive
        Expl = 19,
        /// Goes with (part of a word in a text not well edited)
        Goeswith = 20,
        /// Indirect object
        Iobj = 21,
        /// Marker (word introducing a subordinate clause)
        Mark = 22,
        /// Multi-word expression
        Mwe = 23,
        /// Multi-word verbal expression
        Mwv = 24,
        /// Negation modifier
        Neg = 25,
        /// Noun compound modifier
        Nn = 26,
        /// Noun phrase used as an adverbial modifier
        Npadvmod = 27,
        /// Nominal subject
        Nsubj = 28,
        /// Passive nominal subject
        Nsubjpass = 29,
        /// Numeric modifier of a noun
        Num = 30,
        /// Element of compound number
        Number = 31,
        /// Punctuation mark
        P = 32,
        /// Parataxis relation
        Parataxis = 33,
        /// Participial modifier
        Partmod = 34,
        /// The complement of a preposition is a clause
        Pcomp = 35,
        /// Object of a preposition
        Pobj = 36,
        /// Possession modifier
        Poss = 37,
        /// Postverbal negative particle
        Postneg = 38,
        /// Predicate complement
        Precomp = 39,
        /// Preconjunt
        Preconj = 40,
        /// Predeterminer
        Predet = 41,
        /// Prefix
        Pref = 42,
        /// Prepositional modifier
        Prep = 43,
        /// The relationship between a verb and verbal morpheme
        Pronl = 44,
        /// Particle
        Prt = 45,
        /// Associative or possessive marker
        Ps = 46,
        /// Quantifier phrase modifier
        Quantmod = 47,
        /// Relative clause modifier
        Rcmod = 48,
        /// Complementizer in relative clause
        Rcmodrel = 49,
        /// Ellipsis without a preceding predicate
        Rdrop = 50,
        /// Referent
        Ref = 51,
        /// Remnant
        Remnant = 52,
        /// Reparandum
        Reparandum = 53,
        /// Root
        Root = 54,
        /// Suffix specifying a unit of number
        Snum = 55,
        /// Suffix
        Suff = 56,
        /// Temporal modifier
        Tmod = 57,
        /// Topic marker
        Topic = 58,
        /// Clause headed by an infinite form of the verb that modifies a noun
        Vmod = 59,
        /// Vocative
        Vocative = 60,
        /// Open clausal complement
        Xcomp = 61,
        /// Name suffix
        Suffix = 62,
        /// Name title
        Title = 63,
        /// Adverbial phrase modifier
        Advphmod = 64,
        /// Causative auxiliary
        Auxcaus = 65,
        /// Helper auxiliary
        Auxvv = 66,
        /// Rentaishi (Prenominal modifier)
        Dtmod = 67,
        /// Foreign words
        Foreign = 68,
        /// Keyword
        Kw = 69,
        /// List for chains of comparable items
        List = 70,
        /// Nominalized clause
        Nomc = 71,
        /// Nominalized clausal subject
        Nomcsubj = 72,
        /// Nominalized clausal passive
        Nomcsubjpass = 73,
        /// Compound of numeric modifier
        Numc = 74,
        /// Copula
        Cop = 75,
        /// Dislocated relation (for fronted/topicalized elements)
        Dislocated = 76,
        /// Aspect marker
        Asp = 77,
        /// Genitive modifier
        Gmod = 78,
        /// Genitive object
        Gobj = 79,
        /// Infinitival modifier
        Infmod = 80,
        /// Measure
        Mes = 81,
        /// Nominal complement of a noun
        Ncomp = 82,
    }
}
/// Represents a mention for an entity in the text. Currently, proper noun
/// mentions are supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityMention {
    /// The mention text.
    #[prost(message, optional, tag = "1")]
    pub text: ::std::option::Option<TextSpan>,
    /// The type of the entity mention.
    #[prost(enumeration = "entity_mention::Type", tag = "2")]
    pub r#type: i32,
    /// For calls to [AnalyzeEntitySentiment][] or if
    /// [AnnotateTextRequest.Features.extract_entity_sentiment][google.cloud.language.v1.AnnotateTextRequest.Features.extract_entity_sentiment] is set to
    /// true, this field will contain the sentiment expressed for this mention of
    /// the entity in the provided document.
    #[prost(message, optional, tag = "3")]
    pub sentiment: ::std::option::Option<Sentiment>,
}
pub mod entity_mention {
    /// The supported types of mentions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unknown
        Unknown = 0,
        /// Proper name
        Proper = 1,
        /// Common noun (or noun compound)
        Common = 2,
    }
}
/// Represents an output piece of text.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSpan {
    /// The content of the output text.
    #[prost(string, tag = "1")]
    pub content: std::string::String,
    /// The API calculates the beginning offset of the content in the original
    /// document according to the [EncodingType][google.cloud.language.v1.EncodingType] specified in the API request.
    #[prost(int32, tag = "2")]
    pub begin_offset: i32,
}
/// Represents a category returned from the text classifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationCategory {
    /// The name of the category representing the document, from the [predefined
    /// taxonomy](https://cloud.google.com/natural-language/docs/categories).
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The classifier's confidence of the category. Number represents how certain
    /// the classifier is that this category represents the given text.
    #[prost(float, tag = "2")]
    pub confidence: f32,
}
/// The sentiment analysis request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeSentimentRequest {
    /// Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::std::option::Option<Document>,
    /// The encoding type used by the API to calculate sentence offsets.
    #[prost(enumeration = "EncodingType", tag = "2")]
    pub encoding_type: i32,
}
/// The sentiment analysis response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeSentimentResponse {
    /// The overall sentiment of the input document.
    #[prost(message, optional, tag = "1")]
    pub document_sentiment: ::std::option::Option<Sentiment>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See [Document.language][google.cloud.language.v1.Document.language] field for more details.
    #[prost(string, tag = "2")]
    pub language: std::string::String,
    /// The sentiment for all the sentences in the document.
    #[prost(message, repeated, tag = "3")]
    pub sentences: ::std::vec::Vec<Sentence>,
}
/// The entity-level sentiment analysis request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeEntitySentimentRequest {
    /// Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::std::option::Option<Document>,
    /// The encoding type used by the API to calculate offsets.
    #[prost(enumeration = "EncodingType", tag = "2")]
    pub encoding_type: i32,
}
/// The entity-level sentiment analysis response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeEntitySentimentResponse {
    /// The recognized entities in the input document with associated sentiments.
    #[prost(message, repeated, tag = "1")]
    pub entities: ::std::vec::Vec<Entity>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See [Document.language][google.cloud.language.v1.Document.language] field for more details.
    #[prost(string, tag = "2")]
    pub language: std::string::String,
}
/// The entity analysis request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeEntitiesRequest {
    /// Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::std::option::Option<Document>,
    /// The encoding type used by the API to calculate offsets.
    #[prost(enumeration = "EncodingType", tag = "2")]
    pub encoding_type: i32,
}
/// The entity analysis response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeEntitiesResponse {
    /// The recognized entities in the input document.
    #[prost(message, repeated, tag = "1")]
    pub entities: ::std::vec::Vec<Entity>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See [Document.language][google.cloud.language.v1.Document.language] field for more details.
    #[prost(string, tag = "2")]
    pub language: std::string::String,
}
/// The syntax analysis request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeSyntaxRequest {
    /// Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::std::option::Option<Document>,
    /// The encoding type used by the API to calculate offsets.
    #[prost(enumeration = "EncodingType", tag = "2")]
    pub encoding_type: i32,
}
/// The syntax analysis response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeSyntaxResponse {
    /// Sentences in the input document.
    #[prost(message, repeated, tag = "1")]
    pub sentences: ::std::vec::Vec<Sentence>,
    /// Tokens, along with their syntactic information, in the input document.
    #[prost(message, repeated, tag = "2")]
    pub tokens: ::std::vec::Vec<Token>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See [Document.language][google.cloud.language.v1.Document.language] field for more details.
    #[prost(string, tag = "3")]
    pub language: std::string::String,
}
/// The document classification request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassifyTextRequest {
    /// Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::std::option::Option<Document>,
}
/// The document classification response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassifyTextResponse {
    /// Categories representing the input document.
    #[prost(message, repeated, tag = "1")]
    pub categories: ::std::vec::Vec<ClassificationCategory>,
}
/// The request message for the text annotation API, which can perform multiple
/// analysis types (sentiment, entities, and syntax) in one call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateTextRequest {
    /// Input document.
    #[prost(message, optional, tag = "1")]
    pub document: ::std::option::Option<Document>,
    /// The enabled features.
    #[prost(message, optional, tag = "2")]
    pub features: ::std::option::Option<annotate_text_request::Features>,
    /// The encoding type used by the API to calculate offsets.
    #[prost(enumeration = "EncodingType", tag = "3")]
    pub encoding_type: i32,
}
pub mod annotate_text_request {
    /// All available features for sentiment, syntax, and semantic analysis.
    /// Setting each one to true will enable that specific analysis for the input.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Features {
        /// Extract syntax information.
        #[prost(bool, tag = "1")]
        pub extract_syntax: bool,
        /// Extract entities.
        #[prost(bool, tag = "2")]
        pub extract_entities: bool,
        /// Extract document-level sentiment.
        #[prost(bool, tag = "3")]
        pub extract_document_sentiment: bool,
        /// Extract entities and their associated sentiment.
        #[prost(bool, tag = "4")]
        pub extract_entity_sentiment: bool,
        /// Classify the full document into categories.
        #[prost(bool, tag = "6")]
        pub classify_text: bool,
    }
}
/// The text annotations response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateTextResponse {
    /// Sentences in the input document. Populated if the user enables
    /// [AnnotateTextRequest.Features.extract_syntax][google.cloud.language.v1.AnnotateTextRequest.Features.extract_syntax].
    #[prost(message, repeated, tag = "1")]
    pub sentences: ::std::vec::Vec<Sentence>,
    /// Tokens, along with their syntactic information, in the input document.
    /// Populated if the user enables
    /// [AnnotateTextRequest.Features.extract_syntax][google.cloud.language.v1.AnnotateTextRequest.Features.extract_syntax].
    #[prost(message, repeated, tag = "2")]
    pub tokens: ::std::vec::Vec<Token>,
    /// Entities, along with their semantic information, in the input document.
    /// Populated if the user enables
    /// [AnnotateTextRequest.Features.extract_entities][google.cloud.language.v1.AnnotateTextRequest.Features.extract_entities].
    #[prost(message, repeated, tag = "3")]
    pub entities: ::std::vec::Vec<Entity>,
    /// The overall sentiment for the document. Populated if the user enables
    /// [AnnotateTextRequest.Features.extract_document_sentiment][google.cloud.language.v1.AnnotateTextRequest.Features.extract_document_sentiment].
    #[prost(message, optional, tag = "4")]
    pub document_sentiment: ::std::option::Option<Sentiment>,
    /// The language of the text, which will be the same as the language specified
    /// in the request or, if not specified, the automatically-detected language.
    /// See [Document.language][google.cloud.language.v1.Document.language] field for more details.
    #[prost(string, tag = "5")]
    pub language: std::string::String,
    /// Categories identified in the input document.
    #[prost(message, repeated, tag = "6")]
    pub categories: ::std::vec::Vec<ClassificationCategory>,
}
/// Represents the text encoding that the caller uses to process the output.
/// Providing an `EncodingType` is recommended because the API provides the
/// beginning offsets for various outputs, such as tokens and mentions, and
/// languages that natively use different text encodings may access offsets
/// differently.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EncodingType {
    /// If `EncodingType` is not specified, encoding-dependent information (such as
    /// `begin_offset`) will be set at `-1`.
    None = 0,
    /// Encoding-dependent information (such as `begin_offset`) is calculated based
    /// on the UTF-8 encoding of the input. C++ and Go are examples of languages
    /// that use this encoding natively.
    Utf8 = 1,
    /// Encoding-dependent information (such as `begin_offset`) is calculated based
    /// on the UTF-16 encoding of the input. Java and JavaScript are examples of
    /// languages that use this encoding natively.
    Utf16 = 2,
    /// Encoding-dependent information (such as `begin_offset`) is calculated based
    /// on the UTF-32 encoding of the input. Python is an example of a language
    /// that uses this encoding natively.
    Utf32 = 3,
}
#[doc = r" Generated client implementations."]
pub mod language_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Provides text analysis operations such as sentiment analysis and entity"]
    #[doc = " recognition."]
    pub struct LanguageServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LanguageServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LanguageServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Analyzes the sentiment of the provided text."]
        pub async fn analyze_sentiment(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeSentimentRequest>,
        ) -> Result<tonic::Response<super::AnalyzeSentimentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.language.v1.LanguageService/AnalyzeSentiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Finds named entities (currently proper names and common nouns) in the text"]
        #[doc = " along with entity types, salience, mentions for each entity, and"]
        #[doc = " other properties."]
        pub async fn analyze_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeEntitiesRequest>,
        ) -> Result<tonic::Response<super::AnalyzeEntitiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.language.v1.LanguageService/AnalyzeEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Finds entities, similar to [AnalyzeEntities][google.cloud.language.v1.LanguageService.AnalyzeEntities] in the text and analyzes"]
        #[doc = " sentiment associated with each entity and its mentions."]
        pub async fn analyze_entity_sentiment(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeEntitySentimentRequest>,
        ) -> Result<tonic::Response<super::AnalyzeEntitySentimentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.language.v1.LanguageService/AnalyzeEntitySentiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Analyzes the syntax of the text and provides sentence boundaries and"]
        #[doc = " tokenization along with part of speech tags, dependency trees, and other"]
        #[doc = " properties."]
        pub async fn analyze_syntax(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeSyntaxRequest>,
        ) -> Result<tonic::Response<super::AnalyzeSyntaxResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.language.v1.LanguageService/AnalyzeSyntax",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Classifies a document into categories."]
        pub async fn classify_text(
            &mut self,
            request: impl tonic::IntoRequest<super::ClassifyTextRequest>,
        ) -> Result<tonic::Response<super::ClassifyTextResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.language.v1.LanguageService/ClassifyText",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " A convenience method that provides all the features that analyzeSentiment,"]
        #[doc = " analyzeEntities, and analyzeSyntax provide in one call."]
        pub async fn annotate_text(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnotateTextRequest>,
        ) -> Result<tonic::Response<super::AnnotateTextResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.language.v1.LanguageService/AnnotateText",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for LanguageServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for LanguageServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LanguageServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod language_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with LanguageServiceServer."]
    #[async_trait]
    pub trait LanguageService: Send + Sync + 'static {
        #[doc = " Analyzes the sentiment of the provided text."]
        async fn analyze_sentiment(
            &self,
            request: tonic::Request<super::AnalyzeSentimentRequest>,
        ) -> Result<tonic::Response<super::AnalyzeSentimentResponse>, tonic::Status>;
        #[doc = " Finds named entities (currently proper names and common nouns) in the text"]
        #[doc = " along with entity types, salience, mentions for each entity, and"]
        #[doc = " other properties."]
        async fn analyze_entities(
            &self,
            request: tonic::Request<super::AnalyzeEntitiesRequest>,
        ) -> Result<tonic::Response<super::AnalyzeEntitiesResponse>, tonic::Status>;
        #[doc = " Finds entities, similar to [AnalyzeEntities][google.cloud.language.v1.LanguageService.AnalyzeEntities] in the text and analyzes"]
        #[doc = " sentiment associated with each entity and its mentions."]
        async fn analyze_entity_sentiment(
            &self,
            request: tonic::Request<super::AnalyzeEntitySentimentRequest>,
        ) -> Result<tonic::Response<super::AnalyzeEntitySentimentResponse>, tonic::Status>;
        #[doc = " Analyzes the syntax of the text and provides sentence boundaries and"]
        #[doc = " tokenization along with part of speech tags, dependency trees, and other"]
        #[doc = " properties."]
        async fn analyze_syntax(
            &self,
            request: tonic::Request<super::AnalyzeSyntaxRequest>,
        ) -> Result<tonic::Response<super::AnalyzeSyntaxResponse>, tonic::Status>;
        #[doc = " Classifies a document into categories."]
        async fn classify_text(
            &self,
            request: tonic::Request<super::ClassifyTextRequest>,
        ) -> Result<tonic::Response<super::ClassifyTextResponse>, tonic::Status>;
        #[doc = " A convenience method that provides all the features that analyzeSentiment,"]
        #[doc = " analyzeEntities, and analyzeSyntax provide in one call."]
        async fn annotate_text(
            &self,
            request: tonic::Request<super::AnnotateTextRequest>,
        ) -> Result<tonic::Response<super::AnnotateTextResponse>, tonic::Status>;
    }
    #[doc = " Provides text analysis operations such as sentiment analysis and entity"]
    #[doc = " recognition."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct LanguageServiceServer<T: LanguageService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: LanguageService> LanguageServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for LanguageServiceServer<T>
    where
        T: LanguageService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.language.v1.LanguageService/AnalyzeSentiment" => {
                    #[allow(non_camel_case_types)]
                    struct AnalyzeSentimentSvc<T: LanguageService>(pub Arc<T>);
                    impl<T: LanguageService>
                        tonic::server::UnaryService<super::AnalyzeSentimentRequest>
                        for AnalyzeSentimentSvc<T>
                    {
                        type Response = super::AnalyzeSentimentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AnalyzeSentimentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.analyze_sentiment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AnalyzeSentimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.language.v1.LanguageService/AnalyzeEntities" => {
                    #[allow(non_camel_case_types)]
                    struct AnalyzeEntitiesSvc<T: LanguageService>(pub Arc<T>);
                    impl<T: LanguageService>
                        tonic::server::UnaryService<super::AnalyzeEntitiesRequest>
                        for AnalyzeEntitiesSvc<T>
                    {
                        type Response = super::AnalyzeEntitiesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AnalyzeEntitiesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.analyze_entities(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AnalyzeEntitiesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.language.v1.LanguageService/AnalyzeEntitySentiment" => {
                    #[allow(non_camel_case_types)]
                    struct AnalyzeEntitySentimentSvc<T: LanguageService>(pub Arc<T>);
                    impl<T: LanguageService>
                        tonic::server::UnaryService<super::AnalyzeEntitySentimentRequest>
                        for AnalyzeEntitySentimentSvc<T>
                    {
                        type Response = super::AnalyzeEntitySentimentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AnalyzeEntitySentimentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.analyze_entity_sentiment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AnalyzeEntitySentimentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.language.v1.LanguageService/AnalyzeSyntax" => {
                    #[allow(non_camel_case_types)]
                    struct AnalyzeSyntaxSvc<T: LanguageService>(pub Arc<T>);
                    impl<T: LanguageService>
                        tonic::server::UnaryService<super::AnalyzeSyntaxRequest>
                        for AnalyzeSyntaxSvc<T>
                    {
                        type Response = super::AnalyzeSyntaxResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AnalyzeSyntaxRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.analyze_syntax(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AnalyzeSyntaxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.language.v1.LanguageService/ClassifyText" => {
                    #[allow(non_camel_case_types)]
                    struct ClassifyTextSvc<T: LanguageService>(pub Arc<T>);
                    impl<T: LanguageService> tonic::server::UnaryService<super::ClassifyTextRequest>
                        for ClassifyTextSvc<T>
                    {
                        type Response = super::ClassifyTextResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClassifyTextRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.classify_text(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ClassifyTextSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.language.v1.LanguageService/AnnotateText" => {
                    #[allow(non_camel_case_types)]
                    struct AnnotateTextSvc<T: LanguageService>(pub Arc<T>);
                    impl<T: LanguageService> tonic::server::UnaryService<super::AnnotateTextRequest>
                        for AnnotateTextSvc<T>
                    {
                        type Response = super::AnnotateTextResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AnnotateTextRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.annotate_text(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AnnotateTextSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: LanguageService> Clone for LanguageServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: LanguageService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LanguageService> tonic::transport::NamedService for LanguageServiceServer<T> {
        const NAME: &'static str = "google.cloud.language.v1.LanguageService";
    }
}