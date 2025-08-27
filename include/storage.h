#pragma once

#include <unordered_map>

#include <folly/futures/Future.h>
#include <folly/executors/IOThreadPoolExecutor.h>
#include <folly/Synchronized.h>

class Storage {
public:
    folly::SemiFuture<folly::Unit> store(const std::string& key, const std::string& value) {
        folly::Promise<folly::Unit> promise;
        try {
            _data.withWLock([&](auto& lockedMap) {
                lockedMap[key] = value;
            });
            promise.setValue();
        } catch (...) {
            promise.setException(folly::exception_wrapper(std::current_exception()));
        }

        return promise.getSemiFuture();
    }

    folly::SemiFuture<folly::Optional<std::string>> fetch(const std::string& key) {
        folly::Promise<folly::Optional<std::string>> promise;

        try {
            folly::Optional<std::string> result = _data.withRLock([&](const auto& lockedMap) {
                auto it = lockedMap.find(key);
                if (it != lockedMap.end()) {
                    return folly::make_optional(it->second);
                }
                return folly::Optional<std::string>();
            });

            promise.setValue(std::move(result));
        } catch (...) {
            promise.setException(folly::exception_wrapper(std::current_exception()));
        }

        return promise.getSemiFuture();
    }

private:
    folly::Synchronized<std::unordered_map<std::string, std::string>> _data;
};

std::unique_ptr<folly::Executor> get_executor() {
    return std::make_unique<folly::IOThreadPoolExecutor>(4);
}