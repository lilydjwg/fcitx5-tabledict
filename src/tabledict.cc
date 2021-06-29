#include<libime/table/tablebaseddictionary.h>

using namespace libime;

class TableDict {
public:
  TableBasedDictionary* tbd;
  TableDict();
  ~TableDict();
};

TableDict::TableDict() {
  tbd = new TableBasedDictionary();
}

TableDict::~TableDict() {
  delete tbd;
}

extern "C" {

void put_string(void* data, const char* value, unsigned int len);

TableDict* new_tabledict() {
  auto td = new TableDict();
  return td;
}

void load_main(TableDict* td, const char* filename, void* err) {
  try {
    td->tbd->load(filename);
  } catch (std::exception& e) {
    const char* msg = e.what();
    put_string(err, msg, strlen(msg));
  }
}

void load_user(TableDict* td, const char* filename, void* err) {
  try {
    td->tbd->loadUser(filename);
  } catch (std::exception& e) {
    const char* msg = e.what();
    put_string(err, msg, strlen(msg));
  }
}

void free_table_dict(TableDict* td) {
  delete td;
}

typedef void (*_MyTableMatchCallback)(
  void* data,
  const char* code,
  unsigned int code_len,
  const char* word,
  unsigned int word_len,
  uint32_t index,
  PhraseFlag flag
);

bool match_words(TableDict* td, const char* code, unsigned int size, TableMatchMode mode,
  const _MyTableMatchCallback callback, void* data) {
  auto sv = std::string_view(code, size);
  auto ret = td->tbd->matchWords(
    sv, mode,
    [&data, &callback](
      std::string_view code, std::string_view word, uint32_t index, PhraseFlag flag
    ){
      callback(data, code.data(), code.size(), word.data(), word.size(), index, flag);
      return true;
  });
  return ret;
}

void reverse_lookup(
  TableDict* td, const char* word, unsigned int size, PhraseFlag flag,
   void* result, void* err
) {
  auto sv = std::string_view(word, size);

  try {
    auto r = td->tbd->reverseLookup(sv, flag);
    put_string(result, r.data(), r.size());
  } catch (const std::exception &e) {
    const char* msg = e.what();
    put_string(err, msg, strlen(msg));
  }
}

bool insert(
  TableDict* td, const char* code, int code_len,
  const char* word, int word_len,
  PhraseFlag flag
){
  auto key = std::string_view(code, code_len);
  auto value = std::string_view(word, word_len);
  return td->tbd->insert(key, value, flag);
}

void delete_entry(
  TableDict* td, const char* code, int code_len,
  const char* word, int word_len
){
  auto key = std::string_view(code, code_len);
  auto value = std::string_view(word, word_len);
  return td->tbd->removeWord(key, value);
}

void save_user(TableDict* td, const char* filename, void* err) {
  try {
    td->tbd->saveUser(filename);
  } catch (const std::exception &e) {
    const char* msg = e.what();
    put_string(err, msg, strlen(msg));
  }
}

void statistic(TableDict* td) {
  td->tbd->statistic();
}

}
