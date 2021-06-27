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

TableDict* new_tabledict() {
  auto td = new TableDict();
  return td;
}

char* load_main(TableDict* td, const char* filename) {
  try {
    td->tbd->load(filename);
    return NULL;
  } catch (std::exception& e) {
    char* err = strdup(e.what());
    return err;
  }
}

char* load_user(TableDict* td, const char* filename) {
  try {
    td->tbd->loadUser(filename);
    return NULL;
  } catch (std::exception& e) {
    char* err = strdup(e.what());
    return err;
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

bool match_words(TableDict* td, const char* code, int size, TableMatchMode mode,
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

void statistic(TableDict* td) {
  td->tbd->statistic();
}

}
