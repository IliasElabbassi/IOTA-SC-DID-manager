# // first represent the options
# // 1 - create a did and push it to ISC
# // 2 - update a did
# // 3 - delete a did
# // 4 - read a did
# // bonus - valider un cerficat 
    # - lire dans la chaine le contenu du certificat (deliverance / cles etc)
    # - renvoyer dans le module qui s'occupe de la validation

# // 1 - create a did and push it to ISC
# // cmd to create a did
# // cmd to push the newly did to the ISC

did = ./did_src
wasp-cli chain post-request appenddid addDid String newDID did