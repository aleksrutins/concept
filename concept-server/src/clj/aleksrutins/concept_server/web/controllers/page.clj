(ns aleksrutins.concept-server.web.controllers.page
  (:require
   [ring.util.http-response :as http-response]))

(defn list!
  [req]
  (http-response/ok {:this-route "not implemented"}))

